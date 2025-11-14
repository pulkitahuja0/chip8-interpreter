mod chip8;
mod config;
mod hardware;
mod registers;
mod stack;

// TODO: Config to skip bad opcodes instead of error (don't store in Config struct)

fn main() {
    println!("Hello, world!");

    let cpu: chip8::Chip8;
}
