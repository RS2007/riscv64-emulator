use std::{
    fs::File,
    io::{self, Read, Write},
};

use bus::DRAM_BASE;
use cpu::Cpu;

mod bus;
mod cpu;
mod dram;
#[allow(dead_code, unused_imports)]

fn evaluate(cpu: &mut Cpu, input: &str) -> bool {
    match input {
        "n" | "N" | "" => match cpu.fetch() {
            Ok(inst) => {
                println!("instruction : {:#x}", inst);
                if inst == 0x0 {
                    println!("Program execution done");
                    return false;
                }
                cpu.execute(inst);
                cpu.regs[0] = 0;
                return true;
            }
            Err(_e) => {
                println!("Error");
                return false;
            }
        },
        "regs" | "reg" => {
            cpu.print_registers();
            return true;
        }
        "mem" => {
            cpu.print_mem_around_interest(500);
            return true;
        }
        _ => {
            println!("Invalid instruction for debugger");
            return false;
        }
    }
}

fn main() {
    let mut buffer = vec![];
    if let Ok(mut file) = File::open("test.bin") {
        if let Ok(_) = file.read_to_end(&mut buffer) {
            let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
            core.regs[2] = (DRAM_BASE + 1024 * 1024) as u32;
            loop {
                print!(">");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                input = input.trim().to_string();
                println!("Input: {input}");

                let result = evaluate(&mut core, &input);
                if !result {
                    core.print_registers();
                    return;
                }
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    } else {
        assert!(false, "Should'nt hit this");
    }
}

#[cfg(test)]
mod tests {
    use crate::{bus::DRAM_BASE, cpu::Cpu};

    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn test_add_addi() {
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test_add.bin") {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
                loop {
                    match core.fetch() {
                        Ok(inst) => {
                            println!("instruction : {:#x}", inst);
                            core.execute(inst);
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                }
                assert_eq!(core.regs[0], 0);
                assert_eq!(core.regs[31], 2);
                assert_eq!(core.regs[30], 2);
                assert_eq!(core.regs[29], 4);
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    }

    #[test]
    fn test_load_store() {
        // 8,14
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test_load_store.bin") {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
                loop {
                    match core.fetch() {
                        Ok(inst) => {
                            println!("instruction : {:#x}", inst);
                            if inst == 0x0 {
                                break;
                            }
                            core.execute(inst);
                            core.print_registers();
                            core.print_mem_around_interest(500);
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                }
                core.regs.iter().enumerate().for_each(|(index, reg)| {
                    println!("{index}: {reg}");
                });
                assert_eq!(core.regs[8], 496);
                assert_eq!(core.regs[14], 97);
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    }
    #[test]
    fn test_load_store2() {
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test_load_store2.bin") {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
                loop {
                    match core.fetch() {
                        Ok(inst) => {
                            println!("instruction : {:#x}", inst);
                            if inst == 0x0 {
                                break;
                            }
                            core.execute(inst);
                            core.print_registers();
                            core.print_mem_around_interest(500);
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                }
                core.regs.iter().enumerate().for_each(|(index, reg)| {
                    println!("{index}: {reg}");
                });
                assert_eq!(core.regs[15], 10);
                assert_eq!(core.regs[0], 0);
                assert_eq!(core.regs[1], 0);
                assert_eq!(core.regs[2], 0);
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    }
    #[test]
    fn test_load_store3() {
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test_load_store3.bin") {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
                loop {
                    match core.fetch() {
                        Ok(inst) => {
                            println!("instruction : {:#x}", inst);
                            if inst == 0x0 {
                                break;
                            }
                            core.execute(inst);
                            core.print_registers();
                            core.print_mem_around_interest(500);
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                }
                core.regs.iter().enumerate().for_each(|(index, reg)| {
                    println!("{index}: {reg}");
                });
                assert_eq!(core.regs[11], 4);
                assert_eq!(core.regs[12], 60);
                assert_eq!(core.regs[13], 10);
                assert_eq!(core.regs[14], 60);
                assert_eq!(core.regs[15], 10);
                assert_eq!(core.regs[16], 70);
                assert_eq!(core.regs[17], 70);
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    }

    #[test]
    fn test_load_store4() {
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test_load_store4.bin") {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
                loop {
                    match core.fetch() {
                        Ok(inst) => {
                            println!("instruction : {:#x}", inst);
                            if inst == 0x0 {
                                break;
                            }
                            core.execute(inst);
                            core.print_registers();
                            core.print_mem_around_interest(500);
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                }
                core.regs.iter().enumerate().for_each(|(index, reg)| {
                    println!("{index}: {reg}");
                });
                assert_eq!(core.regs[0], 0);
                assert_eq!(core.regs[1], 259);
                assert_eq!(core.regs[2], 259);
                assert_eq!(core.regs[3], 0);
                assert_eq!(core.regs[4], 259);
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    }

    #[test]
    fn test_slt_family() {
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test_slt_family.bin") {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
                loop {
                    match core.fetch() {
                        Ok(inst) => {
                            println!("instruction : {:#x}", inst);
                            if inst == 0x0 {
                                break;
                            }
                            core.execute(inst);
                            core.print_registers();
                            core.print_mem_around_interest(500);
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                }
                core.regs.iter().enumerate().for_each(|(index, reg)| {
                    println!("{index}: {reg}");
                });
                assert_eq!(core.regs[4], 1);
                assert_eq!(core.regs[5], 0xfffffffd);
                assert_eq!(core.regs[6], 1);
                assert_eq!(core.regs[7], 1);
                assert_eq!(core.regs[8], 1);
                assert_eq!(core.regs[9], 0);
                assert_eq!(core.regs[10], 0xffffffff);
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    }

    #[test]
    fn test_shift_bits_family() {
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test_shift_bits_family.bin") {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
                loop {
                    match core.fetch() {
                        Ok(inst) => {
                            println!("instruction : {:#x}", inst);
                            if inst == 0x0 {
                                break;
                            }
                            core.execute(inst);
                            core.print_registers();
                            core.print_mem_around_interest(500);
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                }
                core.regs.iter().enumerate().for_each(|(index, reg)| {
                    println!("{index}: {reg}");
                });
                assert_eq!(core.regs[0], 0);
                assert_eq!(core.regs[1], 0xffffffff);
                assert_eq!(core.regs[2], 3);
                assert_eq!(core.regs[3], 0xffffffff);
                assert_eq!(core.regs[4], 0x1fffffff);
                assert_eq!(core.regs[5], 0xffffffff);
                assert_eq!(core.regs[6], 0x1fffffff);
                assert_eq!(core.regs[7], 0xfffffffe);
                assert_eq!(core.regs[8], 0xfffffffe);
                assert_eq!(core.regs[9], 0);
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    }

    #[test]
    fn test_fib() {
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test.bin") {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                let mut core = cpu::Cpu::new(dram::Dram::new(buffer));
                core.regs[2] = (DRAM_BASE + 1024 * 1024) as u32;
                loop {
                    match core.fetch() {
                        Ok(inst) => {
                            println!("instruction : {:#x}", inst);
                            if inst == 0x0 {
                                break;
                            }
                            core.execute(inst);
                            core.regs[0] = 0;
                            if core.pc == 0 {
                                assert_eq!(core.regs[10], 5);
                                assert_eq!(core.regs[14], 1);
                                assert_eq!(core.regs[15], 5);
                            }
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                }
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
    }
}
