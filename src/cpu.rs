use crate::{
    bus::{self, *},
    dram::*,
};
#[allow(dead_code, unused_imports)]
pub struct Cpu {
    pub pc: usize,
    pub regs: Vec<u32>,
    pub bus: Bus,
}

const ADD_OPCODE: u32 = 0b0110011;
const ADDI_OPCODE: u32 = 0b0010011;

impl Cpu {
    pub fn new(dram: Dram) -> Self {
        return Cpu {
            pc: DRAM_BASE,
            regs: vec![0; 32],
            bus: bus::Bus::new(dram),
        };
    }
    pub fn fetch(&self) -> Result<u32, ()> {
        match self.bus.load(self.pc, 32) {
            Ok(inst) => Ok(inst as u32),
            Err(_e) => Err(()),
        }
    }
    fn sign_extend(imm: u32) -> u32 {
        let top_bit = (imm >> 11) & 0x1;
        println!("Top bit: {top_bit}");
        return match top_bit {
            0 => imm,
            1 => imm | 0xffffff00,
            _ => {
                assert!(false, "Invalid bit");
                0
            }
        };
    }
    fn alu_add(op1: u32, op2: u32) -> u32 {
        // Need a better implementation that handles hardware behaviour
        return op1.wrapping_add(op2);
    }

    pub fn execute(&mut self, instruction: u32) {
        println!("{:#x}", instruction);
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;
        let rd = ((instruction >> 7) & 0x1f) as usize;
        let opcode = instruction & 0x7f;
        match opcode {
            ADD_OPCODE => {
                println!("Add: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                self.regs[rd] = Cpu::alu_add(self.regs[rs1], self.regs[rs2]);
            }
            ADDI_OPCODE => {
                let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20);
                println!("Add: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                self.regs[rd] = Cpu::alu_add(self.regs[rs1], imm);
            }
            _ => {
                todo!();
            }
        }
    }
}
