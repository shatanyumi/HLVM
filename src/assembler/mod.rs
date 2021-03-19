mod opcode_parsers;
mod oprand_parsers;
mod opcode;

use crate::instruction::Opcode;

#[derive(Debug,PartialEq)]
pub enum Token{
    Op{
        code: Opcode
    },
}