use anyhow::Result;

use crate::{block::Block, opcode::OpCode, Uint256};

#[derive(Debug, Default)]
pub struct Emulator<'a> {
    pub calldata: &'a [u8],
    pub raw_code: Vec<u8>,
    pub code: Vec<Block>,
    pub position: usize,
    pub stack: Vec<Uint256>,
    pub memory: [u8; 32],
    pub return_data: Vec<u8>,
}

impl<'a> Emulator<'a> {
    pub fn new(raw_code: Vec<u8>, code: Vec<Block>, calldata: &'a [u8]) -> Self {
        Self {
            code,
            raw_code,
            calldata,
            ..Default::default()
        }
    }

    pub fn is_end(&self) -> bool {
        !self.return_data.is_empty() || self.position >= self.code.len()
    }

    pub fn current_block(&self) -> &Block {
        &self.code[self.position]
    }

    pub fn get_stack(&self, position: usize) -> Uint256 {
        self.stack
            .get::<usize>(self.stack.len() - position - 1)
            .expect("dupn err")
            .clone()
    }

    pub fn use_stack(&mut self) -> Uint256 {
        self.stack.split_off(self.stack.len() - 1)[0].clone()
    }

    pub fn run(&mut self) -> Result<()> {
        let block = self.current_block().clone();

        match block.opcode {
            OpCode::PUSHN(n, v) => self.eval_pushn(n, v),
            OpCode::DUPN(n) => self.eval_dupn(n)?,
            OpCode::RETURNDATASIZE => self.eval_returndatasize(),
            OpCode::CALLDATALOAD => self.eval_calldataload(),
            OpCode::CODECOPY => self.eval_codecopy(),
            OpCode::RETURN => self.eval_return(),
            OpCode::SAR => self.eval_sar(),
            _ => todo!(),
        }

        self.position += 1;

        Ok(())
    }

    fn eval_pushn(&mut self, _n: u8, value: Uint256) {
        self.stack.push(value);
    }

    fn eval_dupn(&mut self, n: u8) -> Result<()> {
        let value = self.get_stack(n as usize - 1); // TODO: impl Error
        self.stack.push(value);
        Ok(())
    }

    fn eval_returndatasize(&mut self) {
        self.stack.push(Uint256::from(0usize));
    }

    fn eval_calldataload(&mut self) {
        let offset = self.use_stack().try_into().unwrap();
        let v = Uint256::from_bytes_be(&self.calldata[offset..]);
        self.stack.push(v);
    }

    fn eval_codecopy(&mut self) {
        let dest_offset = self.use_stack().try_into().unwrap();
        let offset = self.use_stack().try_into().unwrap();
        let size = self.use_stack().try_into().unwrap();

        let with_zero_padding = {
            let mut v = self.raw_code[offset..].to_vec();
            while v.len() < size {
                v.push(0);
            }
            v
        };

        let mut memory_ = [
            self.memory[..dest_offset].to_vec(),
            with_zero_padding,
            self.memory[dest_offset..].to_vec(),
        ]
        .concat();
        let _ = memory_.split_off(32);

        self.memory = memory_.try_into().unwrap();
    }

    fn eval_return(&mut self) {
        let offset: usize = self.use_stack().try_into().unwrap();
        let size: usize = self.use_stack().try_into().unwrap();

        self.return_data = self.memory[offset..(offset + size)].to_vec();
    }

    fn eval_sar(&mut self) {
        let shift: usize = self.use_stack().try_into().unwrap();
        let value = self.use_stack();
        println!("Shift: {}", shift);
        self.stack.push(value >> shift);
    }
}
