use crate::opcode::OpCode;

pub fn fmt_opcode(op: &OpCode) -> String {
    use OpCode::*;

    match op {
        STOP => "STOP".into(),
        ADD => "ADD".into(), MUL => "MUL".into(),
        SUB => "SUB".into(),
        DIV => "DIV".into(),
        SDIV => "SDIV".into(),
        MOD => "MOD".into(),
        SMOD => "SMOD".into(),
        ADDMOD => "ADDMOD".into(),
        MULMOD => "MULMOD".into(),
        EXP => "EXP".into(),
        SIGNEXTEND => "SIGNEXTEND".into(),
        LT => "LT".into(),
        GT => "GT".into(),
        SLT => "SLT".into(),
        SGT => "SGT".into(),
        EQ => "EQ".into(),
        ISZERO => "ISZERO".into(),
        AND => "AND".into(),
        OR => "OR".into(),
        XOR => "XOR".into(),
        NOT => "NOT".into(),
        BYTE => "BYTE".into(),
        SHL => "SHL".into(),
        SAR => "SAR".into(),
        SHA3 => "SHA3".into(),
        ADDRESS => "ADDRESS".into(),
        BALANCE => "BALANCE".into(),
        ORIGIN => "ORIGIN".into(),
        CALLER => "CALLER".into(),
        CALLVALUE => "CALLVALUE".into(),
        CALLDATALOAD => "CALLDATALOAD".into(),
        CALLDATASIZE => "CALLDATASIZE".into(),
        CALLDATACOPY => "CALLDATACOPY".into(),
        CODESIZE => "CODESIZE".into(),
        CODECOPY => "CODECOPY".into(),
        GASPRICE => "GASPRICE".into(),
        EXTCODESIZE => "EXTCODESIZE".into(),
        EXTCODECOPY => "EXTCODECOPY".into(),
        RETURNDATASIZE => "RETURNDATASIZE".into(),
        RETURNDATACOPY => "RETURNDATACOPY".into(),
        EXTCODEHASH => "EXTCODEHASH".into(),
        BLOCKHASH => "BLOCKHASH".into(),
        COINBASE => "COINBASE".into(),
        TIMESTAMP => "TIMESTAMP".into(),
        NUMBER => "NUMBER".into(),
        DIFFICULTY => "DIFFICULTY".into(),
        GASLIMIT => "GASLIMIT".into(),
        CHAINID => "CHAINID".into(),
        SELFBALANCE => "SELFBALANCE".into(),
        BASEFEE => "BASEFEE".into(),
        POP => "POP".into(),
        MLOAD => "MLOAD".into(),
        MSTORE => "MSTORE".into(),
        MSTORE8 => "MSTORE8".into(),
        SLOAD => "SLOAD".into(),
        SSTORE => "SSTORE".into(),
        JUMP => "JUMP".into(),
        JUMPI => "JUMPI".into(),
        PC => "PC".into(),
        MSIZE => "MSIZE".into(),
        GAS => "GAS".into(),
        JUMPDEST => "JUMPDEST".into(),
        PUSH => "PUSH".into(),
        DUP => "DUP".into(),
        SWAP => "SWAP".into(),
        CREATE => "CREATE".into(),
        CALL => "CALL".into(),
        CALLCODE => "CALLCODE".into(),
        RETURN => "RETURN".into(),
        DELEGATECALL => "DELEGATECALL".into(),
        CREATE2 => "CREATE2".into(),
        STATICCALL => "STATICCALL".into(),
        REVERT => "REVERT".into(),
        SELFDESTRUCT => "SELFDESTRUCT".into(),
        PUSHN(n, v) => format!("PUSH{}\t0x{}", n, v),
        DUPN(n) => format!("DUP{}", n),
        SWAPN(n) => format!("SWAP{}", n),
        LOGN(n) => format!("LOG{}", n),
    }
}

pub fn format<'a>(opcodes: &'a [OpCode]) -> String {
    let mut result = String::new();
    for op in opcodes {
      let fmted_op = fmt_opcode(op);
      result += &format!("{}\n", fmted_op);
    }
    result
}
