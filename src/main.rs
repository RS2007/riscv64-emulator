mod bus;
mod cpu;
mod dram;
#[allow(dead_code, unused_imports)]

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn test_add_addi() {
        let mut buffer = vec![];
        if let Ok(mut file) = File::open("test.bin") {
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
}
