mod opcode_parsers;
mod oprand_parsers;
mod opcode;
mod register_parsers;

use crate::instruction::Opcode;

#[derive(Debug,PartialEq)]
pub enum Token{
    Op{
        code: Opcode
    },
    Register{reg_num: u8}
}
