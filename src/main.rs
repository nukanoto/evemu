use anyhow::Result;
use std::{env, fs, path::Path};

use evm_disasm::parser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let bytecode = if !args[1].is_empty() && Path::new(&args[0]).exists() {
        println!("{}", args[1]);
        let file = Path::new(&args[1]);
        fs::read_to_string(file)?
    } else {
        args[1].to_string()
    };
    
    println!("{:?}", parser::parse(&bytecode));

    Ok(())
}