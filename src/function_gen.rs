#[path = "data_structure.rs"]
mod data_structure;

use data_structure::*;
use std::collections::HashMap;
use std::char;

/*
Generates a single function.
*/
pub fn generate_function(type_map: &HashMap<String, String>,
                         init_map: &HashMap<String, String>,
                         function_name: String,
                         events: &HashMap<String, Vec<String>>,
                         generates: &Vec<Vec<Vec<String>>>,
                         initiates: &Vec<Vec<Vec<String>>>,
                         terminates: &Vec<Vec<Vec<String>>>,
                         public: bool,
                         static_first: bool,)
                         -> Vec<ContractElement> {

    let mut ret : Vec<ContractElement> = Vec::new();

    //For arguments - indexes all live in the same place, so this is how we keep track.
    //These 'arg types' can all be looked up in type map.
    let arg_types : Vec<String> = events.get(&function_name)
        .expect("Trying to generate function that doesn't exist.").clone();
    let mut my_args : Vec<String> = Vec::new();
    let num_args = arg_types.len();
    //Assign letters to each arg.
    let mut char_num = 10;
    for _ in 0..num_args {
        my_args.push(format!("{}", char::from_digit(char_num, 36).unwrap()));
        char_num += 1;
    }

    //Step 1 - Create function declaration block.
    //Types tbd.
    let mut begin : String = format!("function {}(", function_name);
    //Generate rest of begin statement for block:
    //This includes all argument definitions.
    //Skip the first arg if appropriate.
    let arg_start_idx = match static_first && public {
        true => 1,
        false => 0,
    };
    for i in arg_start_idx..my_args.len(){
        let cur_arg = my_args.get(i).unwrap();
        let cur_type = arg_types.get(i).unwrap();
        let cur_sol_type = type_map.get(cur_type).unwrap();
        if i == arg_types.len()-1 {
            begin += &format!("{} {}", cur_sol_type, cur_arg);
        }
        else {
            begin += &format!("{} {}, ", cur_sol_type, cur_arg);
        }
    }
    if public {
        begin += ") public {";
    } else {
        begin += ") private {";
    }

    let end : String = format!("}}");
    let def = ContractElement {
        head : begin,
        tail : end,
        is_block : true,
        is_below : true,
        unindent_after : false,
    };
    ret.push(def);

    //Handle static first.
    if static_first && public {
        let static_arg = my_args.get(0).unwrap();
        let static_type = arg_types.get(0).unwrap();
        let static_sol_type = type_map.get(static_type).unwrap();
        let static_init = init_map.get(static_type).unwrap();
        ret.push(code_line_from_string(format!("{} {} = {};",
                                             static_sol_type,
                                             static_arg,
                                             static_init)));
    }

    //Step 2 - Permissions and empowerment guards.
    ret.append(&mut generate_function_guards(function_name.clone(),
                                             num_args,
                                             &my_args,
                                             public));

    //Step 3 - Initiates
    //TODO Can you initiate a fluent and permissions in the same function? Probs no
    ret.append(&mut generate_function_initiates_terminates(function_name.clone(),
                                                            &my_args,
                                                            initiates,
                                                            false));
    //Step 4 - Generates
    ret.append(&mut generate_function_generates(function_name.clone(),
                                                &my_args,
                                                generates));
    //Step 5 - Terminates
    ret.append(&mut generate_function_initiates_terminates(function_name.clone(),
                                                           &my_args,
                                                           terminates,
                                                           true));

    return ret;
}

fn generate_function_guards(function_name: String,
                            num_args: usize,
                            my_args: &Vec<String>,
                            is_exo: bool)
                            -> Vec<ContractElement> {

    let mut ret: Vec<ContractElement> = Vec::new();
    let mut require : String = format!("require(");

    //Wildcards/maps
    for i in 0..num_args {
        let cur_arg = my_args.get(i).unwrap();
        if is_exo {
            require +=
                &format!("(({}_perm_wildcard_on_{} || {}_perm_map_on_{}[{}]) && ({}_pow_wildcard_on_{} || {}_pow_map_on_{}[{}])) && ",
                         function_name, i,
                         function_name, i, cur_arg,
                         function_name, i,
                         function_name, i, cur_arg);
        } else {
            require +=
                &format!("({}_perm_wildcard_on_{} || {}_perm_map_on_{}[{}]) && ",
                         function_name, i,
                         function_name, i, cur_arg);
        }
    }
    require.pop(); require.pop(); require.pop(); require.pop();
    require += ");";

    ret.push(code_line_from_string(require));
    return ret;
}

fn generate_function_generates(function_name: String,
                               function_args: &Vec<String>,
                               generates: &Vec<Vec<Vec<String>>>)
                               -> Vec<ContractElement> {
    let mut ret: Vec<ContractElement> = Vec::new();

    let mut lines_to_inspect: Vec<Vec<Vec<String>>> = Vec::new();
    for line in generates {
        let first_element = line.get(0).unwrap().get(0).unwrap();
        if first_element == &function_name {
            lines_to_inspect.push(line.clone());
        }
    }

    if lines_to_inspect.len() == 0 {
        //We're done.
        return ret;
    }

    let mut num_ifs = 0; //Needed to make nesting work properly...
    for line in lines_to_inspect {
        //Get args from the front of the statement.
        let mut args: Vec<String> = line.get(0).unwrap().clone();
        args.remove(0);

        //The conditional part of the statement.
        let mut conditional_args: Vec<String> = line.get(line.len()-2).unwrap().clone();
        let has_conditional: bool; //Needed for later.
        //Generate conditional block.
        if conditional_args.len() > 0 {
            has_conditional = true;

            let conditional: String;
            let negation: bool;
            if conditional_args.get(0).unwrap() == "not" {
                negation = true;
                conditional_args.remove(0);
                conditional = String::from(conditional_args
                                           .get(0).unwrap().clone());
                conditional_args.remove(0);
            } else {
                negation = false;
                conditional = String::from(conditional_args
                                           .get(0).unwrap().clone());
                conditional_args.remove(0);
            }
            //TODO Fix IF nesting,
            let mut start: String = format!("if(");
            for arg_idx in 0..conditional_args.len() {
                let arg = conditional_args.get(arg_idx).unwrap().clone();
                //Matching.
                let mut match_idx = 99;
                //Find corresponding index.
                for idx in 0..function_args.len() {
                    let comp = args.get(idx).unwrap();
                    if &arg == comp {
                        match_idx = idx;
                    }
                }
                //TODO Needs generalising and modifying if we want full fluent support.
                if match_idx < 99 {
                    if negation == true {
                        start += &format!("!{}_{}[{}] && ",
                                          conditional,
                                          arg_idx,
                                          function_args.get(match_idx).expect("??"));
                    } else {
                        start += &format!("{}_{}[{}] && ",
                                          conditional,
                                          arg_idx,
                                          function_args.get(match_idx).expect("??"));
                    }
                }
            }
            start.pop();start.pop();start.pop();start.pop();
            start += "){";
            let begin: String = start;
            let end: String = format!("}}");
            let mut def = ContractElement {
                head: begin,
                tail: end,
                is_block: true,
                is_below: match num_ifs {
                    0 => false,
                    _ => true,
                },
                unindent_after: true,
            };
            ret.push(def);
            num_ifs += 1;
        } else {
            has_conditional = false;
        }

        //Ignore first and last parts...
        //Then iterate through to generate fun calls.

       //Split what we're generating into individual words.

        let mut generates: Vec<Vec<String>> = Vec::new();
        let mut generates_cur: Vec<String> = Vec::new();

        for word_idx in 1..line.len()-1 {
            let cur = line.get(word_idx).unwrap().clone();
            for e in cur {
                if e.len() > 1 {
                    generates.push(generates_cur);
                    generates_cur = Vec::new();
                    generates_cur.push(e);
                } else {
                    generates_cur.push(e);
                }
            }
        }
        generates.push(generates_cur);

        //Pick indices...
        let limit: usize;
        if has_conditional {
            limit = generates.len()-1;
        } else {
            limit = generates.len();
        }
        for generates_idx in 1..limit {
            let mut generates_args = generates.get(generates_idx).unwrap().clone();


            //TODO Minor bug with code not generating certain
            //generates - i.e. if there are no ifs.
            //Below line is probably at fault...
            if generates_args.len() < 1 {
                continue;
            }

            let mut generates_call = generates_args.remove(0);
            if generates_call == "not" {
                continue; //Fixes an odd edge case.
            }

            let mut start: String = format!("{}(", generates_call);
            for arg in generates_args {
                //Matching
                let mut match_idx = 99;
                for idx in 0..function_args.len() {
                    let comp = args.get(idx).unwrap();
                    if &arg == comp {
                        match_idx = idx;
                    }
                }
                if match_idx < 99 {
                    start += &format!("{},", function_args.get(match_idx)
                                      .expect("??"));
                } else {
                    panic!("Scary wildcards in 'generates' statement from {}",
                           function_name);
                }
            }
            start.pop();
            start += ");";

            let begin: String = start;
            let end: String = format!("}}");
            let mut def = ContractElement {
                head : begin,
                tail : end,
                is_block : false,
                is_below : false,
                unindent_after: false,
            };
            ret.push(def);
        }
    }

    //Generate function calls.
    return ret;
}

/*
Generates the initiates and terminates calls inside a function.
TODO Solstice pattern matching needs rewriting as it does not support matching in body.
*/
fn generate_function_initiates_terminates(function_name: String,
                                          function_args: &Vec<String>,
                                          lines: &Vec<Vec<Vec<String>>>,
                                          terminates: bool)
                                          -> Vec<ContractElement> {

    let set_to: String = match terminates {
        true => String::from("false"),
        false => String::from("true"),
    };

    let mut ret: Vec<ContractElement> = Vec::new();

    //a statement like:
    //balance_up(U) lines has_money(U);
    //is a line consisting of words (e.g. balance_up(U)) consisting of elements
    //(e.g. balance_up, U)
    //That's how we iterates through the data structure.
    //The choice of data structure here is obv bad - so should refactor.
    for line_idx in 0..lines.len() {
        //Fetch line
        let line = lines.get(line_idx).unwrap();

        //Used for matching args in this line.
        let mut match_args : Vec<String> = Vec::new();

        //Look per word.
        for word_idx in 0..line.len() {
            let word = line.get(word_idx).unwrap();
            //The first word in a block determines if this function lines
            //the following words...
            if word_idx == 0 {
                if word.get(0).unwrap() == &function_name {
                    //'Match_args' are simply the args of the head.
                    match_args = word.clone();
                    match_args.remove(0); //Rmv function name.
                } else {
                    //Not relevant - ignore line
                    break;
                }
            } else {
                //Here is where we do the code generation.
                //We only ever reach this branch for a word IF
                //it is called by this function.
                //TODO Strip out conditionals.
                if word.len() == 0 {
                    //We need this because a bug somewhere means there is an empty
                    //element at the end of everything - this is scary for conditionals.
                    break;
                }

                let first_element = word.get(0).unwrap();

                if first_element == "perm" || first_element == "pow" {
                    //Need to go a level deeper -
                    //split the word down into things we can read.
                    let mut statements : Vec<Vec<String>> = Vec::new();
                    let mut cur_statement : Vec<String> = Vec::new();
                    for element in word {
                        if element == "perm" || element == "pow" {
                            if cur_statement.len() > 0 {
                                statements.push(cur_statement);
                            }
                            cur_statement = Vec::new();
                            cur_statement.push(element.clone());
                        }
                        else {
                            cur_statement.push(element.clone());
                        }
                    }
                    statements.push(cur_statement);

                    for statement in statements {
                        let mut element_args = statement.clone();
                        let type_of_change = element_args.remove(0);
                        let name_of_change = element_args.remove(0);

                        for arg_idx in 0..element_args.len() {
                            let cur_arg = element_args.get(arg_idx).unwrap();
                            //Index through the head args
                            let mut match_pos = 99;
                            for match_arg_idx in 0..match_args.len() {
                                let cur_match_arg = match_args
                                    .get(match_arg_idx)
                                    .unwrap();
                                if cur_arg == cur_match_arg {
                                    match_pos = match_arg_idx;
                                    break;
                                }
                            }
                            let mut out_line : String;
                            if match_pos == 99 {
                                //Must be wildcard.
                                out_line = format!("{}_{}_wildcard_on_{} = {};",
                                                   name_of_change,
                                                   type_of_change,
                                                   arg_idx,
                                                   set_to);
                            } else {
                                let matched_argument = function_args.get(match_pos)
                                    .expect("Initiates/terminates isn't callable!!!");
                                //Needs a mapping.
                                out_line = format!("{}_{}_map_on_{}[{}] = {};",
                                                   name_of_change,
                                                   type_of_change,
                                                   arg_idx,
                                                   matched_argument,
                                                   set_to);
                            }
                            ret.push(code_line_from_string(out_line));
                        }
                    }
                } else {
                    //This branch deals with 'lines'
                    let mut element_args = word.clone();
                    element_args.remove(0);
                    for arg_idx in 0..element_args.len() {
                        let cur_arg = element_args.get(arg_idx).unwrap();
                        //Index through the head args
                        let mut match_pos = 99;
                        for match_arg_idx in 0..match_args.len() {
                            let cur_match_arg = match_args.get(match_arg_idx).unwrap();
                            if cur_arg == cur_match_arg {
                                match_pos = match_arg_idx;
                                break;
                            }
                        }
                        let mut out_line : String = String::from("");
                        let matched_argument = function_args.get(match_pos)
                            .expect("Initiates/terminates isn't callable!!!");
                        for arg_idx in 0..element_args.len() {
                            out_line = format!("{}_{}[{}] = {};",
                                              first_element,
                                              arg_idx,
                                              matched_argument,
                                              set_to);
                        }
                        ret.push(code_line_from_string(out_line));
                    }
                }
            }
        }
    }
    return ret;
}
