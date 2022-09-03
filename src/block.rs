use crate::opcode::OpCode;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Block {
    pub opcode: OpCode,
    pub position: usize,
}

impl Block {
    pub fn new(opcode: OpCode, position: usize) -> Self {
        Self { opcode, position }
    }
}
