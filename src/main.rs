mod cpu;

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn test_add_addi() {
        let mut core = cpu::Cpu::new();
        let mut dram: Box<Vec<u8>> = Box::new(vec![]);
        if let Ok(mut file) = File::open("test.bin") {
            if let Ok(_) = file.read_to_end(&mut dram) {
                core.add_dram(dram);
            } else {
                assert!(false, "Should'nt hit this");
            }
        } else {
            assert!(false, "Should'nt hit this");
        }
        core.dram.iter().for_each(|byte| println!("{:#02x}", byte));
        while core.pc < core.dram.len() {
            let inst = core.fetch();
            core.execute(inst);
            core.pc += 4;
        }
        assert_eq!(core.regs[0], 0);
        assert_eq!(core.regs[31], 2);
        assert_eq!(core.regs[30], 3);
        assert_eq!(core.regs[29], 5);
    }
}
