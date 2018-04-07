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
pub fn file_to_string(path: String) -> String {
    let mut file = File::open(path).expect("Error opening file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file.");
    return contents;
}
/*
Load parse from json file.
 */
#[allow(dead_code)]
pub fn get_json_parse(path : String) -> Value {
    let mut contents = file_to_string(path);
    //Remove extra square brackets...
    contents.remove(0);
    contents.pop();
    let v : Value = serde_json::from_str(&contents).unwrap();
    return v;
}
/*
Convert institution name to string.
*/
#[allow(dead_code)]
pub fn get_inst_name(parse : &Value) -> String {
    let mut data : String = format!("{}",
                                    parse["contents"]["names"]["institution"]);
    //Trim quote marks.
    data.remove(0);
    data.pop();
    return data;
}
/*
Fetches relationship between parts of a parse.
*/
#[allow(dead_code)]
pub fn fetch_relationship_from_parse(parse: &Value, relationship: &str) -> Vec<Vec<Vec<String>>> {
    let mut i = 0;
    let mut j = 0;

    let mut vecs : Vec<Vec<Vec<String>>> = Vec::new();

    loop {
        let mut loop_vec : Vec<Vec<String>> = Vec::new();
        let cur_parse = &parse["contents"][relationship][i];
        let check : String = format!("{}", cur_parse);

        if check == "null" {
            break;
        } else {
            loop {
                let cur_s : String = format!("{}", cur_parse[j]);
                if cur_s == "null" {
                    break;
                } else {
                    loop_vec.push(strip_parse_structure(cur_s));
                    j += 1;
                }
            }
            j = 0;
            i += 1;
            vecs.push(loop_vec);
        }
    }

    return vecs;
}
/*
Get names from blocks that are formatted like
{"create_contract":["User"],"transfer":["User","User"]}
*/
#[allow(dead_code)]
pub fn get_names_from_block(parse: &Value, block: &str) -> Vec<String> {
    let parse_str = format!("{}", parse["contents"][block]);
    let parse_str = parse_str.replace("{", "!");

    let mut names : Vec<String> = Vec::new();
    //let re = Regex::new("[,{{]\"(.*?)\":").unwrap();
    let re = Regex::new("(!|])(.*?)\":").unwrap();
    for cap in re.captures_iter(&parse_str) {
        let mut cur_cap = String::from(&cap[0]);
        names.push(cur_cap);
    }
    return trim_unwanted_chars(names);
}
/*
Requires you to pass the result of get_names_from_block.
Gets a map of types from block name.
Lets us deconstruct i.e. inst and ex events.
*/
#[allow(dead_code)]
pub fn get_types_from_block(parse: &Value, block: &str, names: &Vec<String>)
                        -> HashMap<String, Vec<String>> {
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
/*
Maps types based on manual type set file.
First hashmap maps Instal -> Sol
Second hashmap maps Instal Init -> Literal val
This function is fragile.
Type set file isn't checked or anything. If its bad, you'll just get bad code
or an error.
 */
#[allow(dead_code)]
pub fn map_types(path : String) -> Vec<HashMap<String, String>> {
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


/*
Strips parse structure from a string - just grabs everything in quote marks and shoves it in a list.
*/
#[allow(dead_code)]
fn strip_parse_structure(s : String) -> Vec<String> {
    let mut ret : Vec<String> = Vec::new();

    let re = Regex::new("\"(.*?)\"").unwrap();
    let iter = re.captures_iter(&s);

    for cap in iter {
        let out_str : String = String::from(cap.get(1).unwrap().as_str());
        ret.push(out_str);
    }

    return ret;
}
#[allow(dead_code)]
fn trim_unwanted_chars(vec: Vec<String>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let unwanted = vec!["[", "]", "!", "\"", ",", ":"];
    for s in &vec {
        let mut new = s.clone();
        for c in &unwanted {
            new = new.replace(c, "");
        }
        new.trim();
        ret.push(new);
    }
    return ret;
}
