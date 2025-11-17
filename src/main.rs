use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::{Parser, command};

use crate::{chip8::Chip8, config::Config};

mod chip8;
mod config;
mod hardware;
mod registers;
mod stack;
mod timers;

#[derive(Parser)]
#[command(name = "CHIP-8 Interpreter")]
#[command(about = "A CHIP-8 interpreter for the terminal", long_about = None)]
#[command(next_line_help = true)]
struct Args {
    file: PathBuf,
    #[arg(long, default_value_t = true)]
    #[arg(help = "Use BNNN behavior instead of BXNN")]
    bnnn: bool,
    #[arg(long, default_value_t = false)]
    #[arg(help = "Skip stack underflow errors (returning subroutines from an empty stack)")]
    skip_stack_underflow: bool,
    #[arg(long, default_value_t = false)]
    #[arg(help = "Set VF to 1 if I + VX > 0xFFF")]
    flag_fx1e_overflow: bool,
    #[arg(long, default_value_t = false)]
    #[arg(help = "Ignore Y for 8XY6 and 8XYE shifts")]
    shift_in_place_8xy: bool,
    #[arg(long, default_value_t = false)]
    #[arg(help = "Increment I by X + 1 after FX55 and FX65")]
    increment_i_on_mem: bool,
    #[arg(long, default_value_t = false)]
    #[arg(help = "Skip invalid opcodes instead of crashing program")]
    skip_bad_opcodes: bool,
    #[arg(long, default_value_t = 500)]
    #[arg(help = "Set the instruction speed in Hz")]
    cpu_hz: u32,
}

// TODO: Config to set speed of execution (timers and cycles)
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
        bxnn: !args.bnnn,
        fx1e_overflow: args.flag_fx1e_overflow,
        shift_in_place_8xy: args.shift_in_place_8xy,
        increment_i_on_mem: args.increment_i_on_mem,
    };

    let mut cpu = Chip8::new(&buffer, config);

    let cycle_duration = Duration::from_secs_f32(1.0 / args.cpu_hz as f32);
    let mut last_cycle = Instant::now();

    loop {
        let elapsed = last_cycle.elapsed();
        if elapsed < cycle_duration {
            std::thread::sleep(cycle_duration - elapsed);
        }
        last_cycle = Instant::now();
        match cpu.step() {
            Ok(()) => {}
            Err(err) => {
                if !args.skip_bad_opcodes {
                    panic!("Err: {}", err)
                }
            }
        }
    }
}
