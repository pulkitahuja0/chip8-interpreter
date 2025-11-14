use std::{fs::File, io::Read, path::PathBuf};

use clap::{Parser, command};

use crate::{chip8::Chip8, config::Config};

mod chip8;
mod config;
mod hardware;
mod registers;
mod stack;

#[derive(Parser)]
#[command(name="Chip8 Interpreter")]
struct Args {
    
    file: PathBuf
}

// TODO: Config to skip bad opcodes instead of error (don't store in Config struct)
fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    let mut buffer = [0u8; 3584];
    let mut file = File::open(&args.file)
        .unwrap_or_else(|e| panic!("Failed to open file {}: {}", args.file.display(), e));

    let bytes_read = file.read(&mut buffer).unwrap_or_else(|e| panic!("Failed to read file {}: {}", args.file.display(), e));


    let config = Config {
        skip_stack_underflow: false,
        bxnn: true,
        fx1e_overflow: false,
        shift_in_place_8xy: false,
        increment_i_on_mem: false
    };

    let mut cpu= Chip8::new(&buffer, config);

    loop {
        match cpu.step() {
            Ok(()) => {},
            Err(err) => {
                panic!("Err: {}", err)
            }
        }
    }
}
