use anyhow::Result;

use crate::{block::Block, opcode::OpCode};

#[derive(Debug, Default)]
pub struct Emulator {
    pub code: Vec<Block>,
    pub position: usize,
    pub stack: Vec<u8>,
}

impl Emulator {
    pub fn new(code: Vec<Block>) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }

    pub fn is_end(&self) -> bool {
        self.position >= self.code.len()
    }

    pub fn current_block(&self) -> &Block {
        &self.code[self.position]
    }

    pub fn run(&mut self) -> Result<()> {
        let block = self.current_block().clone();

        match block.opcode {
            OpCode::PUSHN(n, v) => self.eval_pushn(n, v),
            OpCode::DUPN(n) => self.eval_dupn(n)?,
            _ => todo!(),
        }

        self.position += 1;

        Ok(())
    }

    fn eval_pushn(&mut self, _n: u8, mut value: Vec<u8>) {
        self.stack.append(&mut value);
    }

    fn eval_dupn(&mut self, n: u8) -> Result<()> {
        let value = *self.stack.get::<usize>((n - 1).into()).expect("dupn err"); // TODO: impl Error
        self.stack.push(value);
        Ok(())
    }
}
