use std::{fs::File, io::Read, path::PathBuf};

use clap::{Parser, command};

use crate::{chip8::Chip8, config::Config};

mod chip8;
mod config;
mod hardware;
mod registers;
mod stack;

#[derive(Parser)]
#[command(name = "Chip8 Interpreter")]
struct Args {
    file: PathBuf,
    #[arg(long, default_value_t = true)]
    bxnn: bool,
    #[arg(long, default_value_t = false)]
    skip_stack_underflow: bool,
    #[arg(long, default_value_t = false)]
    flag_fx1e_overflow: bool,
    #[arg(long, default_value_t = false)]
    shift_in_place_8xy: bool,
    #[arg(long, default_value_t = false)]
    increment_i_on_mem: bool,
}

// TODO: Config to skip bad opcodes instead of error (don't store in Config struct)
fn main() {
    let args = Args::parse();
    let mut buffer = [0u8; 3584];
    let mut file = File::open(&args.file)
        .unwrap_or_else(|e| panic!("Failed to open file {}: {}", args.file.display(), e));

    let _bytes_read = file
        .read(&mut buffer)
        .unwrap_or_else(|e| panic!("Failed to read file {}: {}", args.file.display(), e));

    let config = Config {
        skip_stack_underflow: args.skip_stack_underflow,
        bxnn: args.bxnn,
        fx1e_overflow: args.flag_fx1e_overflow,
        shift_in_place_8xy: args.shift_in_place_8xy,
        increment_i_on_mem: args.increment_i_on_mem,
    };

    let mut cpu = Chip8::new(&buffer, config);

    loop {
        match cpu.step() {
            Ok(()) => {}
            Err(err) => {
                panic!("Err: {}", err)
            }
        }
    }
}
