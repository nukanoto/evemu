use anyhow::Result;

use std::{env, fs, path::Path};

use evm_disasm::{formatter, parser, emulator::Emulator};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let bytecode = if !args[1].is_empty() && Path::new(&args[0]).exists() {
        let file = Path::new(&args[1]);
        fs::read_to_string(file)?
    } else {
        args[1].to_string()
    };

    let parsed = parser::parse(&bytecode);
    print!("{}", formatter::format(&parsed));
    let mut emu = Emulator::new(parsed);
    println!("Stack: {:02x?}", emu.stack);
    while !emu.is_end() {
        println!("---------");
        print!("{}", formatter::format(&[emu.current_block().clone()]));
        emu.run()?;
        println!("Stack: {:02x?}", emu.stack);
    }

    Ok(())
}
