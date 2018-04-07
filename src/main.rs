/*
'Solstice' - this tool converts an InstAL parse (+ extra hint data) to an Ethereum
Solidity contract that is valid when run through solc, probably.

Author: Zachary Shannon
 */

extern crate serde;
extern crate serde_json;
extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod data_structure;
mod parse;
mod contract_gen;
use data_structure::*;
use parse::*;
use contract_gen::*;

/*
Jumping off point.
*/
fn main() {
    //Load parse data.
    let type_map_file_path = String::from("settype.typ");
    let json_file_path = String::from("parse.json");
    let parse = get_json_parse(json_file_path);

    let contract = generate_contract_code(&parse, type_map_file_path, true);
    let output = contract_to_string(contract);

    //https://rustbyexample.com/std_misc/file/create.html
    let out_path = Path::new("o.sol");
    let mut out_file : File = match File::create(&out_path) {
        Err(why) => panic!("Could not create out file: {}", why),
        Ok(out_file) => out_file,
    };
    match out_file.write_all(output.as_bytes()) {
        Err(why) => panic!("Couldn't write to out file: {}", why),
        Ok(_) => println!("Done."),
    }
}
