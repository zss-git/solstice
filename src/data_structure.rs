pub struct ContractElement {
    pub head : String,
    pub tail : String,
    pub is_block : bool,
    pub is_below : bool,
    pub unindent_after: bool,
}
//Probably should add an automatically generated id and check against that...
impl PartialEq for ContractElement {
    fn eq(&self, other: &ContractElement) -> bool {
        if self.tail == other.tail && self.head == other.head{
            return true;
        } else {
            return false;
        }
    }
}

/*
Converts list of contract elements to a string for output.
Includes correct indentation.
Simply by putting ContractElements in order and using the tail and block parts
correctly, this function will automatically indent and close your blocks.
*/
#[allow(dead_code)]
pub fn contract_to_string(contract : Vec<ContractElement>) -> String {
    let mut ret : String = String::new();
    let mut indents = 0;

    let empty_block = ContractElement {
        head : String::from(""),
        tail : String::from(""),
        is_block : true,
        is_below : false,
        unindent_after: false,
    };


    //List of blocks to ignore when we go back in reverse.
    let mut last_block_out: &ContractElement = &empty_block;
    let mut num_unindent_afters = 0;
    let mut last_block : &ContractElement = &empty_block;
    for i in 0..contract.len() {
        let mut e : &ContractElement = contract.get(i).expect("Array index oob");
        if e.is_block && e.is_below {
            //Close block above...
            indents -= 1;
            for _i in 0..indents {
                ret = ret + "  ";
            }
            if !e.unindent_after && num_unindent_afters > 0 {
                ret = ret + &last_block.tail + "\n";
                indents -= 1;
                for _i in 0..indents {
                    ret = ret + "  ";
                }
                ret = ret + &last_block_out.tail + "\n";
                num_unindent_afters = 0;
            } else if e.unindent_after {
                num_unindent_afters += 1;
                last_block = e;
                ret = ret + &last_block.tail + "\n";
            } else {
                ret = ret + &last_block.tail + "\n";
                last_block = e;
            }
            //Now write block below.
            for _i in 0..indents {
                ret = ret + "  ";
            }
            ret = ret + &e.head + "\n";
            indents += 1;
        } else if e.is_block {
            for _i in 0..indents {
                ret = ret + "  ";
            }
            ret = ret + &e.head + "\n";
            indents += 1;

            if e.unindent_after && num_unindent_afters == 0 {
                num_unindent_afters += 1;
                num_unindent_afters += 1;
                last_block_out = last_block;
                last_block = e;
            } else if e.unindent_after {
                num_unindent_afters += 1;
                last_block = e;
            } else {
                last_block = e;
            }
        } else {
            for _i in 0..indents {
                ret = ret + "  ";
            }
            ret = ret + &e.head + "\n";
        }
    }
    for e in contract.iter().rev() {
        if e.is_block && e.is_below {
            //Skip closing blocks under these conditions -
            //they have already been closed.
            continue;
        } else if indents == 0 {
            //We've obviously gone too far.
            continue;
        }
        else if e.is_block {
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
#[allow(dead_code)]
pub fn code_line_from_string(s: String) -> ContractElement {
    let end : String = String::from("");

    let def = ContractElement {
        head : s,
        tail : end,
        is_block : false,
        is_below : false,
        unindent_after: false,
    };
    return def;
}
