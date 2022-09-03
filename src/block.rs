use crate::opcode::OpCode;

// TODO: raw opcode
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block<'a> {
    pub opcode: OpCode<'a>,
    pub position: usize,
}

impl<'a> Block<'a> {
    pub fn new(opcode: OpCode<'a>, position: usize) -> Self {
        Self { opcode, position }
    }
}
