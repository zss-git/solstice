/*
'Solstice' - this tool converts an InstAL parse (+ extra hint data) to an Ethereum
Solidity contract that is valid when run through solc, probably.

Author: Zachary Shannon
 */

extern crate serde;
extern crate serde_json;
extern crate regex;

use serde_json::Value;

use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;

use regex::Regex;

/*
Load file at a file path to a string.
*/
fn file_to_string(path: String) -> String {
    let mut file = File::open(path).expect("Error opening file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file.");
    return contents;
}

/*
Load parse from json file.
 */
fn get_json_parse(path : String) -> Value {
    let mut contents = file_to_string(path);
    //Remove extra square brackets...
    contents.remove(0);
    contents.pop();
    let v : Value = serde_json::from_str(&contents).unwrap();
    return v;
}

/*
Convert instal types from json to rust vector.
 */
fn types_to_vec(parse : &Value) -> Vec<String> {
    //Trim curly braces
    let mut data : String = format!("{}", parse["contents"]["types"]);
    data.remove(0);
    data.pop();

    let mut vec : Vec<String> = Vec::new();

    //Split on commas
    let comma_split = data.split(",");
    for s in comma_split {
        //Split on colon
        let mut colon_split = s.split(":");
        let mut cur_name = String::from(colon_split.nth(0).unwrap());
        //Strip quote marks
        cur_name.remove(0);
        cur_name.pop();
        vec.push(cur_name);
    }

    return vec;
}

/*
Convert institution name to string.
*/
fn institution_name_to_string(parse : &Value) -> String {
    let mut data : String = format!("{}",
                                    parse["contents"]["names"]["institution"]);
    //Trim quote marks.
    data.remove(0);
    data.pop();
    return data;
}

/*
Maps types based on manual type set file.
First hashmap maps Instal -> Sol
Second hashmap maps Instal Init -> Literal val
This function is fragile.
Type set file isn't checked or anything. If its bad, you'll just get bad code
or an error.
 */
fn map_types(path : String) -> Vec<HashMap<String, String>> {
    let mut ret = Vec::new();
    let mut type_map : HashMap<String, String> = HashMap::new(); 
    let mut init_map : HashMap<String, String> = HashMap::new();
    let cont = file_to_string(path);

    //Split on newline
    let newline_split = cont.split("\n");
    for s in newline_split {
        //Fragile - these unwraps basically assume format is good.
        //Split on bang - first half tells us which map
        let mut bang_split = s.split("!");
        let map_type = String::from(bang_split.next().unwrap().trim());

        //Next half is the actual map - split on colon.
        //This is needed but not 100% sure why we get a None here and not earlier.
        //TODO Work this out.
        let mut mapping : String;
        let next_val = bang_split.next();
        match next_val {
            Some(st) => mapping = String::from(st),
            None => break,
        }

        let mut colon_split = mapping.split(":");

        //Store mapping.
        let inst_type = String::from(colon_split.next().unwrap().trim());
        let sol_type = String::from(colon_split.next().unwrap().trim());
        if map_type == "Type"{
            type_map.insert(inst_type, sol_type);
        }
        else {
            init_map.insert(inst_type, sol_type);
        }
    }
    ret.push(type_map);
    ret.push(init_map);
    return ret;
}

struct ContractElement {
    head : String,
    tail : String,
    is_block : bool,
}

/*
Generate the preamble using the institution name.
*/
fn generate_preamble(parse: &Value) -> ContractElement {
    let inst_name = institution_name_to_string(parse);
    //Escape { or } with another { or } in Rust.
    let begin : String = format!("contract {} {{", inst_name);
    let end : String = String::from("}");

    let preamble = ContractElement {
        head : begin,
        tail : end,
        is_block : true,
    };
    return preamble;
}

/*
Get names from blocks that are formatted like
{"create_contract":["User"],"transfer":["User","User"]}
*/
fn get_names_from_block(parse: &Value, block: &str) -> Vec<String> {
    let parse_str = format!("{}", parse["contents"][block]);

    let mut names : Vec<String> = Vec::new();
    let re = Regex::new("[,{{]\"(.*?)\":").unwrap();
    for cap in re.captures_iter(&parse_str) {
        let mut cur_cap = String::from(&cap[0]);
        cur_cap.pop();
        cur_cap.pop();
        cur_cap.remove(0);
        cur_cap.remove(0);
        cur_cap.trim();
        names.push(cur_cap);
    }
    return names;
}

/*
Requires you to pass the result of get_names_from_block.
Gets a map of types from block name.
Lets us deconstruct i.e. inst and ex events.
*/
fn get_types_from_block(parse: &Value, block: &str, names: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map : HashMap<String, Vec<String>> =  HashMap::new();

    for key in names {
        let mut i = 0;
        let mut val : Vec<String> = Vec::new();
        loop {
            let mut cur_val = format!("{}", parse["contents"][block][&key][i]);
            if cur_val == "null"{
                break;
            } else {
                cur_val.pop();
                cur_val.remove(0);
                cur_val.trim();
                val.push(cur_val);
            }
            i += 1;
        }
        map.insert(key.clone(), val);
    }
    return map;
}

fn string_to_code_line(s: String) -> ContractElement {
    let end : String = String::from("");

    let def = ContractElement {
        head : s,
        tail : end,
        is_block : false,
    };
    return def;
}

fn generate_global_elements(parse: &Value) -> Vec<ContractElement> {
    let mut ret = Vec::new();

    let inst_name = institution_name_to_string(parse);

    //Signature. Assumes just one thing in signature.
    //This is the type of the argument to the contract init function.
    //TODO Fix that.
    let mut create_contract : String = format!("{}", parse["contents"]["exevents"]["create_contract"]);
    create_contract.pop();
    create_contract.pop();
    create_contract.remove(0);
    create_contract.remove(0);
    create_contract.trim();

    //Create permission and power maps
    let ex_event_names : Vec<String> = get_names_from_block(parse, "exevents");
    let in_event_names : Vec<String> = get_names_from_block(parse, "inevents");

    let ex_events = get_types_from_block(parse, "exevents", &ex_event_names);

    for name in ex_event_names {
        let types = ex_events.get(&name).expect("Missing type map for exevent.");
        let mut pos = 0;
        for t in types {
            ret.push(string_to_code_line(format!("bool {}_wildcard_on_{} = false;", name, pos)));
            pos += 1;
        }
    }

    //Permissions and powers.
    //Wildcards must be irrelevant here.
    //println!("initials");
    let mut loop_it = 0;
    loop {
        let parse_initials = format!("{}", parse["contents"]["initials"][loop_it]);
        loop_it+=1;
        if parse_initials == "null"{
            break;
        } else {
            //println!("{}", parse_initials);
        }
    }

    return ret;
}

/*
Generate the contract creation function.
*/
fn generate_creation_fun(parse: &Value, init_map : HashMap<String, String>) -> Vec<ContractElement> {
    let mut ret = Vec::new();

    let inst_name = institution_name_to_string(parse);

    //Signature. Assumes just one thing in signature.
    //This is the type of the argument to the contract init function.
    //TODO Fix that.
    let mut create_contract : String = format!("{}", parse["contents"]["exevents"]["create_contract"]);
    create_contract.pop();
    create_contract.pop();
    create_contract.remove(0);
    create_contract.remove(0);
    create_contract.trim();

    let begin : String = format!("contract {} {{", inst_name);
    let end : String = String::from("}");

    let def = ContractElement {
        head : begin,
        tail : end,
        is_block : true,
    };
    ret.push(def);

    return ret;
}

/*
Generate final contract code.
*/
fn generate_contract_code(parse : &Value,
                          type_map_file_path : String) -> Vec<ContractElement>{

    let mut contract = Vec::new();

    //Step 1 - Preamble.
    contract.push(generate_preamble(parse));

    //Step 2 - Types.
    let types : Vec<String> = types_to_vec(parse);
    let mut maps : Vec<HashMap<String, String>> = map_types(type_map_file_path);
    let init_map : HashMap<String, String> = maps.pop().unwrap();
    let type_map : HashMap<String, String> = maps.pop().unwrap();

    println!("Type mappings are as follows:");
    for t in types{
        //Error handling if there is no match.
        match type_map.get(&t) {
            Some(st) => println!("{} -> {}", t, st),
            None => panic!("No match for type {} in map file.", t)
        }
    }
    println!("End type mappings.\n");

    //Step 3 - Global
    contract.append(&mut generate_global_elements(parse));

    //Step 4 - Contract Creation function.
    contract.append(&mut generate_creation_fun(parse, init_map));

    return contract;
}

/*
Converts list of contract elements to a string for output.
TODO Indentation.
*/
fn contract_to_string(contract : Vec<ContractElement>) -> String {
    let mut ret : String = String::new();
    let mut indents = 0;
    for e in &contract {
        if e.is_block {
            for _i in 0..indents {
                ret = ret + "  ";
            }
            ret = ret + &e.head + "\n";
            indents += 1;
        } else {
            for _i in 0..indents {
                ret = ret + "  ";
            }
            ret = ret + &e.head + "\n";
        }
    }
    for e in contract.iter().rev() {
        if e.is_block {
            indents -= 1;
            for _i in 0..indents {
                ret = ret + "  ";
            }
            ret = ret + &e.tail + "\n";
        } else {
            //Ignore tail on non-blocks.
        }
    }
    return ret;
}

/*
Jumping off point.
*/
fn main() {
    //Load parse data.
    let type_map_file_path = String::from("settype.typ");
    let json_file_path = String::from("parse.json");
    let parse = get_json_parse(json_file_path);

    let contract = generate_contract_code(&parse, type_map_file_path);
    let output = contract_to_string(contract);

    println!("{}", output);
}
