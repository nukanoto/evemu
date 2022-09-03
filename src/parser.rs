use nom::{
    bytes::complete::{take_while_m_n},
    combinator::{map_res},
    multi::many0,
    IResult,
};
use nom_locate::{LocatedSpan};

use crate::{block::Block, opcode::OpCode};

type Span<'a> = LocatedSpan<&'a str>;

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn parse_hex_u8(input: Span) -> IResult<Span, u8> {
    let input_str = input.fragment();
    let raw: IResult<&str, u8> = map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input_str);
    
    raw
        .map(|r| {
            (
                unsafe {
                    Span::new_from_raw_offset(
                        input.location_offset() + 2,
                        input.location_line(),
                        r.0,
                        (),
                    )
                },
                r.1,
            )
        })
        .map_err(|e| {
            e.map_input(|e| unsafe {
                Span::new_from_raw_offset(
                    input.location_offset() + 2,
                    input.location_line(),
                    e,
                    (),
                )
            })
        })
}

fn parse_opcode(input: Span) -> IResult<Span, OpCode> {
    let (input, op) = parse_hex_u8(input)?;

    let (input, result) = match op {
        0x00 => (input, OpCode::STOP),
        0x01 => (input, OpCode::ADD),
        0x02 => (input, OpCode::MUL),
        0x03 => (input, OpCode::SUB),
        0x04 => (input, OpCode::DIV),
        0x05 => (input, OpCode::SDIV),
        0x06 => (input, OpCode::MOD),
        0x07 => (input, OpCode::SMOD),
        0x08 => (input, OpCode::ADDMOD),
        0x09 => (input, OpCode::MULMOD),
        0x0a => (input, OpCode::EXP),
        0x0b => (input, OpCode::SIGNEXTEND),
        0x10 => (input, OpCode::LT),
        0x11 => (input, OpCode::GT),
        0x12 => (input, OpCode::SLT),
        0x13 => (input, OpCode::SGT),
        0x14 => (input, OpCode::EQ),
        0x15 => (input, OpCode::ISZERO),
        0x16 => (input, OpCode::AND),
        0x17 => (input, OpCode::OR),
        0x18 => (input, OpCode::XOR),
        0x19 => (input, OpCode::NOT),
        0x1a => (input, OpCode::BYTE),
        0x1b => (input, OpCode::SHL),
        0x1c => (input, OpCode::SAR),
        0x20 => (input, OpCode::SHA3),
        0x30 => (input, OpCode::ADDRESS),
        0x31 => (input, OpCode::BALANCE),
        0x32 => (input, OpCode::ORIGIN),
        0x33 => (input, OpCode::CALLER),
        0x34 => (input, OpCode::CALLVALUE),
        0x35 => (input, OpCode::CALLDATALOAD),
        0x36 => (input, OpCode::CALLDATASIZE),
        0x37 => (input, OpCode::CALLDATACOPY),
        0x38 => (input, OpCode::CODESIZE),
        0x39 => (input, OpCode::CODECOPY),
        0x3a => (input, OpCode::GASPRICE),
        0x3b => (input, OpCode::EXTCODESIZE),
        0x3c => (input, OpCode::EXTCODECOPY),
        0x3d => (input, OpCode::RETURNDATASIZE),
        0x3e => (input, OpCode::RETURNDATACOPY),
        0x3f => (input, OpCode::EXTCODEHASH),
        0x40 => (input, OpCode::BLOCKHASH),
        0x41 => (input, OpCode::COINBASE),
        0x42 => (input, OpCode::TIMESTAMP),
        0x43 => (input, OpCode::NUMBER),
        0x44 => (input, OpCode::DIFFICULTY),
        0x45 => (input, OpCode::GASLIMIT),
        0x46 => (input, OpCode::CHAINID),
        0x47 => (input, OpCode::SELFBALANCE),
        0x48 => (input, OpCode::BASEFEE),
        0x50 => (input, OpCode::POP),
        0x51 => (input, OpCode::MLOAD),
        0x52 => (input, OpCode::MSTORE),
        0x53 => (input, OpCode::MSTORE8),
        0x54 => (input, OpCode::SLOAD),
        0x55 => (input, OpCode::SSTORE),
        0x56 => (input, OpCode::JUMP),
        0x57 => (input, OpCode::JUMPI),
        0x58 => (input, OpCode::PC),
        0x59 => (input, OpCode::MSIZE),
        0x5a => (input, OpCode::GAS),
        0x5b => (input, OpCode::JUMPDEST),
        0xb0 => (input, OpCode::PUSH),
        0xb1 => (input, OpCode::DUP),
        0xb2 => (input, OpCode::SWAP),
        0xf0 => (input, OpCode::CREATE),
        0xf1 => (input, OpCode::CALL),
        0xf2 => (input, OpCode::CALLCODE),
        0xf3 => (input, OpCode::RETURN),
        0xf4 => (input, OpCode::DELEGATECALL),
        0xf5 => (input, OpCode::CREATE2),
        0xfa => (input, OpCode::STATICCALL),
        0xfd => (input, OpCode::REVERT),
        0xff => (input, OpCode::SELFDESTRUCT),
        _ => {
            let mut input = input;
            let result: OpCode;

            if (0x60..0x80).contains(&op) {
                // PUSH1-32
                let n = op - 0x60 + 1;
                let (input_, value) =
                    take_while_m_n((n * 2).into(), (n * 2).into(), is_hex_digit)(input)?;
                input = input_;
                result = OpCode::PUSHN(n, value.fragment());
            } else if (0x80..0x90).contains(&op) {
                // DUP1-32
                let n = op - 0x80 + 1;
                result = OpCode::DUPN(n);
            } else if (0x90..0xA0).contains(&op) {
                // SWAP1-32
                let n = op - 0x90 + 1;
                result = OpCode::SWAPN(n);
            } else if (0xA0..0xA5).contains(&op) {
                // LOG1-32
                let n = op - 0xA0 + 1;
                result = OpCode::LOGN(n);
            } else {
                result = OpCode::INVALID(op)
            }

            (input, result)
        }
    };

    Ok((input, result))
}

fn parse_block(span: Span) -> IResult<Span, Block> {
    let position = span.location_offset() / 2;
    let (span, opcode) = parse_opcode(span)?;
    let block = Block::new(opcode, position);
    Ok((span, block))
}

fn parse_root(span: Span) -> IResult<Span, Vec<Block>> {
    let input = span;
    let (input, result) = many0(parse_block)(input)?;
    Ok((input, result))
}

pub fn parse(input: &str) -> Vec<Block> {
    let span = Span::new(input);
    let (_, parsed) = parse_root(span).expect("Failed to parse"); // TODO: remove expect
    parsed
}

#[cfg(test)]
mod tests {
    use crate::block::Block;

    use super::{parse, OpCode::*};

    #[test]
    fn test_parse() {
        /*
        #define macro MAIN() = takes(0) returns(0) {
            0x00 calldataload     // [number1] // load first 32 bytes onto the stack - number 1
            0x20 calldataload     // [number2] // load second 32 bytes onto the stack - number 2
            add                   // [number1+number2] // add number 1 and 2 and put the result onto the stack

            0x00 mstore           // place [number1 + number2] in memory
            0x20 0x00 return      // return the result
        }
         */
        let bytecode = "600f8060093d393df36000356020350160005260206000f3";
        let parsed = parse(bytecode);
        assert_eq!(
            parsed,
            vec![
                Block {
                    opcode: PUSHN(1, "0f"),
                    position: 0
                },
                Block {
                    opcode: DUPN(1),
                    position: 2
                },
                Block {
                    opcode: PUSHN(1, "09"),
                    position: 3
                },
                Block {
                    opcode: RETURNDATASIZE,
                    position: 5
                },
                Block {
                    opcode: CODECOPY,
                    position: 6
                },
                Block {
                    opcode: RETURNDATASIZE,
                    position: 7
                },
                Block {
                    opcode: RETURN,
                    position: 8
                },
                Block {
                    opcode: PUSHN(1, "00"),
                    position: 9
                },
                Block {
                    opcode: CALLDATALOAD,
                    position: 11
                },
                Block {
                    opcode: PUSHN(1, "20"),
                    position: 12
                },
                Block {
                    opcode: CALLDATALOAD,
                    position: 14
                },
                Block {
                    opcode: ADD,
                    position: 15
                },
                Block {
                    opcode: PUSHN(1, "00"),
                    position: 16
                },
                Block {
                    opcode: MSTORE,
                    position: 18
                },
                Block {
                    opcode: PUSHN(1, "20"),
                    position: 19
                },
                Block {
                    opcode: PUSHN(1, "00"),
                    position: 21
                },
                Block {
                    opcode: RETURN,
                    position: 23
                }
            ]
        );
    }
}
