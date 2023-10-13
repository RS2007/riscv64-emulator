mod bus;
mod cpu;
mod dram;
#[allow(dead_code, unused_imports)]

fn main() {}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    use super::*;
    use std::{fs::File, io::Read};

    fn dump_registers(core: &Cpu) {
        println!("Printing regs");
        println!("---------------------");
        core.regs.iter().enumerate().for_each(|(index, reg)| {
            println!("{index}: {reg}");
        });
        println!("---------------------");
        println!("Register ends");
    }

    fn print_mem_around_interest(core: &Cpu, addr: usize) {
        println!("Printing mem");
        println!("-----------------------");
        for i in (addr - 40)..=addr {
            println!("{i} {}", core.bus.load(i, 8).unwrap());
        }
        println!("-----------------------");
        println!("Mem ends");
    }
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
                            core.pc += 4;
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
                            dump_registers(&core);
                            print_mem_around_interest(&core, 500);
                            core.pc += 4;
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
                            dump_registers(&core);
                            print_mem_around_interest(&core, 500);
                            core.pc += 4;
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
                            dump_registers(&core);
                            print_mem_around_interest(&core, 500);
                            core.pc += 4;
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
                            dump_registers(&core);
                            print_mem_around_interest(&core, 500);
                            core.pc += 4;
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
                            dump_registers(&core);
                            print_mem_around_interest(&core, 500);
                            core.pc += 4;
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
                            dump_registers(&core);
                            print_mem_around_interest(&core, 500);
                            core.pc += 4;
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
}
