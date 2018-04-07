extern crate serde;
extern crate serde_json;
extern crate regex;

use serde_json::Value;

use regex::Regex;
use regex::Captures;

use std::char;
use std::collections::HashMap;

#[path = "data_structure.rs"]
mod data_structure;

#[path = "parse.rs"]
mod parse;

#[path = "function_gen.rs"]
mod function_gen;

use data_structure::*;
use parse::*;
use self::function_gen::*;

/*
Generate final contract code.

Set 'static_first' to true if you wish to set the first argument using the init map, for exo
functions.

This is useful to make sure that the 'caller' of an exogenous event function is always
the literal caller in solidity, for example.
If this is not set, it will have to be done manually AFTER code generation.
*/
pub fn generate_contract_code(parse : &Value,
                              type_map_file_path : String,
                              static_first: bool)
                              -> Vec<ContractElement>{

    let mut contract = Vec::new();

    //Fetch generates/initiates
    let generates : Vec<Vec<Vec<String>>>
        = fetch_relationship_from_parse(parse, "generates");
    let initiates : Vec<Vec<Vec<String>>>
        = fetch_relationship_from_parse(parse, "initiates");
    let terminates : Vec<Vec<Vec<String>>>
        = fetch_relationship_from_parse(parse, "terminates");

    //Step 1 - Preamble.
    contract.push(generate_preamble(parse));

    //Step 2 - Types.
    let mut maps : Vec<HashMap<String, String>> = map_types(type_map_file_path);
    let init_map : HashMap<String, String> = maps.pop().unwrap();
    let type_map : HashMap<String, String> = maps.pop().unwrap();

    //Step 3 - Global
    contract.append(&mut generate_global_elements(parse, &type_map));

    //Step 4 - Violations
    contract.append(&mut generate_violates(parse, &type_map));

    //Step 5 - Contract Creation function.
    contract.append(&mut generate_creation_fun(parse, &type_map, &generates,
                                               &init_map, static_first));

    //Step 6 - Generate functions.
    contract.append(&mut generate_functions(parse, &type_map, &init_map,
                                            &generates, &initiates,
                                            &terminates, static_first));
    return contract;
}
/*
TODO Refactor - this function is huge.
It works, but thats pretty much where any positives about it end.
*/
fn generate_global_elements(parse: &Value,
                            type_map: &HashMap<String, String>)
                            -> Vec<ContractElement> {
    let mut ret = Vec::new();

    //let inst_name = get_inst_name(parse);

    //Signature. Assumes just one thing in signature.
    //This is the type of the argument to the contract init function.
    //TODO Fix that.
    let mut create_contract : String
        = format!("{}", parse["contents"]["exevents"]["create_contract"]);
    create_contract.pop();
    create_contract.pop();
    create_contract.remove(0);
    create_contract.remove(0);
    create_contract.trim();

    //Create permission and power maps/representations.
    let ex_event_names : Vec<String> = get_names_from_block(parse, "exevents");
    let in_event_names : Vec<String> = get_names_from_block(parse, "inevents");

    let ex_events = get_types_from_block(parse, "exevents", &ex_event_names);
    let in_events = get_types_from_block(parse, "inevents", &in_event_names);

    //Initial Permissions and powers.
    //Problem - we need to keep track and then set when we declare.
    let mut initials : Vec<String> = Vec::new();

    //TODO This needs to be moved to the 'parse' module.
    let mut loop_it = 0;
    loop {
        let parse_initials = format!("{}", parse["contents"]["initials"][loop_it]);
        loop_it+=1;
        if parse_initials == "null"{
            break;
        } else {
            //TODO NO support for conditionals. Here is where it would go.
            //TODO This needs to really be shipped off
            let re = Regex::new("\"(.*?)\"").unwrap();
            let mut iter = re.captures_iter(&parse_initials);
            let cap : Captures = match iter.next() {
                Some(c) => c,
                None => panic!("Poorly formatted initial"),
            };
            //This is the type - either perm or pow.
            let init_type : String = String::from(cap.get(1).unwrap().as_str());
            let cap : Captures = match iter.next() {
                Some(c) => c,
                None => panic!("Poorly formatted initial"),
            };
            //This is the name of the event which we use.
            let event_name : String = String::from(cap.get(1).unwrap().as_str());

            //Generate vector entries.
            initials.push(format!("{}{}", event_name, init_type));
            }
    }

    //Explain what we're doing.
    ret.push(code_line_from_string(format!("//Permissions and empowerments.")));
    ret.push(code_line_from_string(format!("//Exogenous events.")));

    //For each name, get its associate types. Then for each type, create a boolean that
    //indicates either wildcard is on or a specific relationship.
    //Have one for both permission and power.
    for name in ex_event_names {
        let types = ex_events.get(&name).expect("Missing type map for exevent.");
        let mut pos = 0;
        for t in types {
            if initials.contains(&format!("{}perm", name)){
                ret.push(code_line_from_string(format!("bool {}_perm_wildcard_on_{} = true;", name, pos)));
            } else {
                ret.push(code_line_from_string(format!("bool {}_perm_wildcard_on_{} = false;", name, pos)));
            }

            if initials.contains(&format!("{}pow", name)){
                ret.push(code_line_from_string(format!("bool {}_pow_wildcard_on_{} = true;", name, pos)));
            } else {
                ret.push(code_line_from_string(format!("bool {}_pow_wildcard_on_{} = false;", name, pos)));
            }

            //Now do full lists.
            let map_from : String = match type_map.get(t) {
                Some(s) => s.clone(),
                None => panic!("No value for type {} in type map.", t),
            };

            ret.push(code_line_from_string(format!("mapping ({} => bool) private {}_perm_map_on_{};", map_from, name, pos)));
            ret.push(code_line_from_string(format!("mapping ({} => bool) private {}_pow_map_on_{};", map_from, name, pos)));
            pos += 1;
        }
    }

    //Insert an empty line.
    ret.push(code_line_from_string(format!("")));

    ret.push(code_line_from_string(format!("//Inst events.")));
    //Repeat for inevents.
    for name in in_event_names {
        let types = in_events.get(&name).expect("Missing type map for exevent.");
        let mut pos = 0;
        for t in types {
            if initials.contains(&format!("{}perm", name)){
                ret.push(code_line_from_string(format!("bool {}_perm_wildcard_on_{} = true;", name, pos)));
            } else {
                ret.push(code_line_from_string(format!("bool {}_perm_wildcard_on_{} = false;", name, pos)));
            }

            if initials.contains(&format!("{}pow", name)){
                ret.push(code_line_from_string(format!("bool {}_pow_wildcard_on_{} = true;", name, pos)));
            } else {
                ret.push(code_line_from_string(format!("bool {}_pow_wildcard_on_{} = false;", name, pos)));
            }

            //Now do full lists.
            let map_from : String = match type_map.get(t) {
                Some(s) => s.clone(),
                None => panic!("No value for type {} in type map.", t),
            };

            ret.push(code_line_from_string(format!("mapping ({} => bool) private {}_perm_map_on_{};", map_from, name, pos)));
            ret.push(code_line_from_string(format!("mapping ({} => bool) private {}_pow_map_on_{};", map_from, name, pos)));
            pos += 1;
        }
    }

    ret.push(code_line_from_string(format!("")));
    ret.push(code_line_from_string(format!("//Fluents")));

    let fluent_names : Vec<String> = get_names_from_block(parse, "fluents");
    let fluent_types : HashMap<String, Vec<String>> = get_types_from_block(parse,
                                                                           "fluents",
                                                                           &fluent_names);
    for name in fluent_names {
        let args = fluent_types.get(&name).expect("Fluent doesn't exist");
        for arg_idx in 0..args.len() {
            ret.push(code_line_from_string(format!("mapping(address => bool) public {}_{};",
                                                 name,
                                                 arg_idx)));
        }
    }
    ret.push(code_line_from_string(format!("")));

    return ret;
}
/*
Generate the contract creation function.
*/
fn generate_creation_fun(parse: &Value,
                         type_map: &HashMap<String, String>,
                         generates: &Vec<Vec<Vec<String>>>,
                         init_map: &HashMap<String, String>,
                         static_first: bool)
                         -> Vec<ContractElement> {
    let mut ret = Vec::new();

    let inst_name = get_inst_name(parse);

    //Signature. Assumes just one thing in signature.
    //This is the type of the argument to the contract init function.
    //TODO Fix that.
    let ex_event_names : Vec<String> = get_names_from_block(parse, "exevents");
    let ex_events = get_types_from_block(parse, "exevents", &ex_event_names);

    let mut begin : String = format!("function {}(", inst_name);

    //This char_num nonsense counts from a-z
    let mut char_num = 10;
    let types = ex_events.get("create_contract").expect("No create contract function");

    let first_arg_type = types.get(0).unwrap();
    for typ in types {
        if !static_first {
            begin += &format!("{} {}",
                              type_map.get(typ).expect("Error with type map"),
                              char::from_digit(char_num, 36).unwrap());
        }
        char_num += 1;
    }

    begin += ") public {";

    let end : String = String::from("}");

    let def = ContractElement {
        head : begin,
        tail : end,
        is_block : true,
        is_below : false,
        unindent_after: false,
    };
    ret.push(def);

    //Look for create_contract, see what it generates
    //(will generate inst_creation, presumably) and create appropriate fun calls.
    ret.push(code_line_from_string(format!("//Function calls for initial 'create_contract' event.")));

    //This block of code probably doesn't do anything particularly useful,
    //if we're honest with ourselves. In theory it should be extendable, though.
    for block in generates {
        if block[0][0] == "create_contract" {
            //Do matching.
            let mut head_block : Vec<String> = block[0].clone();
            head_block.remove(0);

            for i in 1..(block.len()-2) {
                let fun = block.get(i).expect("Array out of bounds");
                let mut fun_call : String = format!("{}(", fun.get(0).expect("No function name."));

                for j in 1..fun.len() {
                    //Check for a match.
                    //Copy previous letter assignment technique.
                    let arg = fun.get(j).expect("Malformed arg list");
                    let mut char_num = 10;
                    for head_arg in &head_block {
                        if arg == head_arg {
                            if char_num == 10 && static_first {
                                fun_call += &format!("{}", init_map.get(first_arg_type).unwrap());
                            } else {
                                fun_call += &format!("{}", char::from_digit(char_num, 36).unwrap());
                            }
                        }
                        char_num +=1;
                    }
                }
                fun_call += ");";
                ret.push(code_line_from_string(format!("{}", fun_call)));

            }

        }
    }

    return ret;
}
/*
*/
fn generate_functions(parse: &Value,
                      type_map: &HashMap<String, String>,
                      init_map: &HashMap<String, String>,
                      generates: &Vec<Vec<Vec<String>>>,
                      initiates: &Vec<Vec<Vec<String>>>,
                      terminates: &Vec<Vec<Vec<String>>>,
                      static_first: bool)
                      -> Vec<ContractElement> {

    let mut ret: Vec<ContractElement> = Vec::new();

    let ex_event_names: Vec<String> = get_names_from_block(parse, "exevents");
    let in_event_names: Vec<String> = get_names_from_block(parse, "inevents");

    let ex_events = get_types_from_block(parse, "exevents", &ex_event_names);
    let in_events = get_types_from_block(parse, "inevents", &in_event_names);

    for event in in_event_names {
        ret.append(&mut generate_function(type_map, init_map,
                                          event, &in_events,
                                          generates, initiates,
                                          terminates, false, static_first));
    }
    for event in ex_event_names {
        ret.append(&mut generate_function(type_map, init_map,
                                          event, &ex_events,
                                          generates,initiates,
                                          terminates, true, static_first));
    }

    return ret;
}
/*
*/
fn generate_violates(parse: &Value,
                     type_map: &HashMap<String, String>)
                     -> Vec<ContractElement> {
    let mut ret: Vec<ContractElement> = Vec::new();
    ret.push(code_line_from_string(String::from("//Violation events")));

    //probs loop here
    let violation_names : Vec<String> = get_names_from_block(parse, "vievents");
    let violation_events = get_types_from_block(parse, "vievents", &violation_names);

    for name in violation_names {
        let mut begin: String = format!("event {}(", name);

        let mut char_num = 10;
        let args = violation_events.get(&name).unwrap();
        for idx in 0..args.len() {
            let arg = args.get(idx).unwrap();
            let arg_type = type_map.get(arg).expect("Mismatch in type map - violates");
            begin += &format!("{} {}, ",
                              arg_type,
                              char::from_digit(char_num, 36).unwrap());
            char_num += 1;
        }
        begin.pop(); begin.pop();
        begin += ");";
        let end : String = format!("}}");
        let def = ContractElement {
            head : begin,
            tail : end,
            is_block : false,
            is_below : false,
            unindent_after : false,
        };
        ret.push(def);
        ret.push(code_line_from_string(String::from("")));
    }


    return ret;
}
/*
Generate the preamble using the institution name.
*/
fn generate_preamble(parse: &Value) -> ContractElement {
    let inst_name = get_inst_name(parse);
    //Escape { or } with another { or } in Rust.
    let begin : String = format!("//Code automatically generated by SOLSTICE\npragma solidity ^0.4.19;\ncontract {} {{", inst_name);
    let end : String = String::from("}");

    let preamble = ContractElement {
        head : begin,
        tail : end,
        is_block : true,
        is_below : false,
        unindent_after: false,
    };
    return preamble;
}
