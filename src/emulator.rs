use anyhow::Result;

use crate::{block::Block, opcode::OpCode, util::Uint256Util, Uint256};

// TODO: use program counter
#[derive(Debug, Default)]
pub struct Emulator<'a> {
    pub calldata: &'a [u8],
    pub raw_code: Vec<u8>,
    pub code: Vec<Block>,
    pub block_index: usize,
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
        !self.return_data.is_empty() || self.block_index >= self.code.len()
    }

    pub fn current_block(&self) -> &Block {
        &self.code[self.block_index]
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
            OpCode::SHR => self.eval_shr(),
            OpCode::EQ => self.eval_eq(),
            OpCode::JUMPI => self.eval_jumpi()?,
            OpCode::JUMPDEST => self.eval_jumpdest(),
            _ => todo!(),
        }

        self.block_index += 1;

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
        let v = Uint256::from_bytes_be(&self.calldata[offset..offset + 32]);
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

    fn eval_shr(&mut self) {
        let shift: usize = self.use_stack().try_into().unwrap();
        let value = self.use_stack();

        if shift > 255 {
            self.stack.push(0u32.into())
        } else {
            self.stack.push(value >> shift);
        }
    }

    fn eval_eq(&mut self) {
        let left = self.use_stack();
        let right = self.use_stack();

        self.stack.push(((left == right) as usize).into())
    }

    fn eval_jumpi(&mut self) -> Result<()> {
        let counter: usize = self.use_stack().try_into()?;
        let b = self.use_stack();

        if !(b == 0usize.into()) {
            self.block_index = self
                .code
                .iter()
                .position(|x| x.position == counter)
                .unwrap()
                - 1;
        }

        Ok(())
    }

    fn eval_jumpdest(&mut self) {}

    fn _eval_add(&mut self) {
        /*
        let l = self.use_stack();
        let r = self.use_stack();
        */
        todo!()
    }
}
