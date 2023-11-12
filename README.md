## Risc-V 32I Emulator

- A functional RV32I emulator that supports all the RV32I instructions except FENCE,ECALL and CSR. 

### Tests
- Test files start with and `test_` and are written in `asm` except for `test.c`(fibonacci numbers) which is written in C
- A `Makefile` is used to create RV32 executables that can run on the emulator 
    - `make` compiles the C and ASM program to RV32 flat binaries.(`*.bin` files)
- `cargo test` executes these binaries on the emulator and compares the register state to the values obtained running the executables in [Spike](https://github.com/riscv-software-src/riscv-isa-sim)

### How to run my RV32 programs on the emulator 
1. Follow the makefile instructions to compile `C` or `asm` programs to a flat binary
2. In line 49 in `./main.rs`, change the executable name to the name of your binary.

> [!WARNING]
> By default the emulator runs in debug mode, you can use `n`,`N` or `Enter` to proceed emulation and `reg` or `regs` to view the status of the registers and `mem` to view the memory dump

### TODO
- This is part of a complete RV64 emulator project can run XV6
- Further improvements on this branch:
    - Make a CLI interface for ease of access(optinos for directly executing C and ASM files)
    - Make debug mode optional
    - Add support for CSR,FENCE and ECALL