# CHIP-8 INTERPRETER

A CHIP-8 interpreter working right inside of your terminal.

## Building

```
git clone https://github.com/pulkitahuja0/chip8-interpreter
cd chip8-interpreter
cargo build
```

## Usage

```
chip8-interpreter --help

A CHIP-8 interpreter for the terminal

Usage: chip8-interpreter.exe [OPTIONS] <FILE>

Arguments:
  <FILE>


Options:
      --bnnn
          Use BNNN behavior instead of BXNN
      --skip-stack-underflow
          Skip stack underflow errors (returning subroutines from an empty stack)
      --flag-fx1e-overflow
          Set VF to 1 if I + VX > 0xFFF
      --shift-in-place-8xy
          Ignore Y for 8XY6 and 8XYE shifts
      --increment-i-on-mem
          Increment I by X + 1 after FX55 and FX65
      --skip-bad-opcodes
          Skip invalid opcodes instead of crashing program
  -h, --help
          Print help
```