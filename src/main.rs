use anyhow::Result;

use std::{env, fs, path::Path};

use evm_utils::{emulator::Emulator, formatter, parser};

// TODO: add CLI
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let bytecode = if !args[1].is_empty() && Path::new(&args[1]).exists() {
        let file = Path::new(&args[1]);
        fs::read_to_string(file)?
    } else {
        args[1].to_string()
    };
    let bytecode = bytecode.trim();

    let parsed = parser::parse(bytecode);
    print!("{}", formatter::format(&parsed));
    let calldata = hex::decode("0f52d66e00000000000000000000000000000000000000000000000000000000000000640000000000000000000000000000000000000000000000000000000000000064").unwrap();
    let mut emu = Emulator::new(
        hex::decode(bytecode).expect("Invalid bytecode"),
        parsed,
        &calldata,
    );
    println!("Stack: {:02x?}", emu.stack);
    println!("Memory: {:02x?}", emu.memory);
    while !emu.is_end() {
        println!("---------");
        print!("{}", formatter::format(&[emu.current_block().clone()]));
        emu.run()?;
        println!("Stack: {:02x?}", emu.stack);
        println!("Memory: {:02x?}", emu.memory);
        println!("Return data: {:02x?}", emu.return_data);
    }

    Ok(())
}
