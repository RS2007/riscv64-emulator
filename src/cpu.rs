#[allow(dead_code)]
pub struct Cpu {
    pub dram: Box<Vec<u8>>,
    pub pc: usize,
    pub regs: Vec<u32>,
}

const ADD_OPCODE: u32 = 0b0110011;
const ADDI_OPCODE: u32 = 0b0010011;

impl Cpu {
    pub fn new() -> Self {
        return Cpu {
            dram: Box::new(vec![]),
            pc: 0x0,
            regs: vec![0; 32],
        };
    }
    pub fn add_dram(&mut self, dram: Box<Vec<u8>>) {
        self.dram = dram;
    }
    pub fn fetch(&self) -> u32 {
        return (self.dram[self.pc + 3] as u32) << 24
            | (self.dram[self.pc + 2] as u32) << 16
            | (self.dram[self.pc + 1] as u32) << 8
            | (self.dram[self.pc] as u32);
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
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;
        let rd = ((instruction >> 7) & 0x1f) as usize;
        let opcode = instruction & 0x7f;
        match opcode {
            ADD_OPCODE => {
                self.regs[rd] = Cpu::alu_add(self.regs[rs1], self.regs[rs2]);
            }
            ADDI_OPCODE => {
                let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20);
                self.regs[rd] = Cpu::alu_add(self.regs[rs1], imm);
            }
            _ => {
                todo!();
            }
        }
    }
}
