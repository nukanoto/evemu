use num_bigint::BigUint;

pub type Uint256 = BigUint;

pub mod block;
pub mod emulator;
pub mod formatter;
pub mod opcode;
pub mod parser;
pub mod util;
