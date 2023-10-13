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

const ALU_OPCODE: u32 = 0b0110011;
const ALUI_OPCODE: u32 = 0b0010011;
const ADD_FUNCT7: u32 = 0b0000000;
const SUB_FUNCT7: u32 = 0b0100000;
const ADDI_FUNCT3: u32 = 0b000;
const ORI_FUNCT3: u32 = 0b110;
const LOAD_OPCODE: u32 = 0b0000011;
const STORE_OPCODE: u32 = 0b0100011;
const SB_FUNCT3: u32 = 0b000;
const SH_FUNCT3: u32 = 0b001;
const SW_FUNCT3: u32 = 0b010;
const LB_FUNCT3: u32 = 0b000;
const LH_FUNCT3: u32 = 0b001;
const LW_FUNCT3: u32 = 0b010;
const LBU_FUNCT3: u32 = 0b100;
const LHU_FUNCT3: u32 = 0b101;

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
            Ok(inst) => match inst {
                0 => Err(()),
                _ => Ok(inst as u32),
            },
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
        return op1.wrapping_add(op2);
    }
    fn alu_sub(op1: u32, op2: u32) -> u32 {
        return op1.wrapping_sub(op2);
    }

    pub fn execute(&mut self, instruction: u32) {
        println!("{:#x}", instruction);
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;
        let rd = ((instruction >> 7) & 0x1f) as usize;
        let opcode = instruction & 0x7f;
        let funct3 = ((instruction >> 12) & 0x7) as u32;
        let funct7 = ((instruction >> 25) & 0x7f) as u32;
        match opcode {
            ALU_OPCODE => match funct7 {
                ADD_FUNCT7 => {
                    println!("Add: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                    self.regs[rd] = Cpu::alu_add(self.regs[rs1], self.regs[rs2]);
                }
                SUB_FUNCT7 => {
                    println!("Sub: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                    self.regs[rd] = Cpu::alu_sub(self.regs[rs1], self.regs[rs2]);
                }
                _ => {
                    todo!();
                }
            },
            ALUI_OPCODE => match funct3 {
                ADDI_FUNCT3 => {
                    let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20);
                    println!("Add: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                    self.regs[rd] = Cpu::alu_add(self.regs[rs1], imm);
                }
                ORI_FUNCT3 => {
                    let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20);
                    println!("Or: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                    self.regs[rd] = self.regs[rs1] | imm;
                }
                _ => {
                    todo!();
                }
            },
            LOAD_OPCODE => match funct3 {
                LB_FUNCT3 => {
                    println!("Load Byte");
                    self.regs[rd] = Cpu::sign_extend(
                        (self
                            .bus
                            .load(
                                (self.regs[rs1]
                                    + Cpu::sign_extend((instruction & 0xfff00000) >> 20))
                                    as usize,
                                8,
                            )
                            .unwrap()
                            & 0xff) as u32,
                    )
                }
                LH_FUNCT3 => {
                    println!("Load Half Word");
                    self.regs[rd] = Cpu::sign_extend(
                        (self
                            .bus
                            .load(
                                (self.regs[rs1]
                                    + Cpu::sign_extend((instruction & 0xfff00000) >> 20))
                                    as usize,
                                16,
                            )
                            .unwrap()
                            & 0xffff) as u32,
                    )
                }
                LW_FUNCT3 => {
                    println!("Load Word");
                    println!(
                        "rs1 = {}:{}, This value gets loaded: {:?}",
                        rs1,
                        self.regs[rs1],
                        (self.bus.load(
                            Cpu::alu_add(
                                self.regs[rs1],
                                Cpu::sign_extend((instruction >> 20) & 0xfff)
                            ) as usize,
                            32,
                        ))
                        .unwrap()
                    );
                    self.regs[rd] = Cpu::sign_extend(
                        (self
                            .bus
                            .load(
                                Cpu::alu_add(
                                    self.regs[rs1],
                                    Cpu::sign_extend((instruction >> 20) & 0xfff),
                                ) as usize,
                                32,
                            )
                            .unwrap()
                            & 0xffffffff) as u32,
                    )
                }
                LBU_FUNCT3 => {
                    println!("LBU");
                    self.regs[rd] = (self
                        .bus
                        .load(
                            Cpu::alu_add(
                                self.regs[rs1],
                                Cpu::sign_extend((instruction & 0xfff00000) >> 20),
                            ) as usize,
                            8,
                        )
                        .unwrap()
                        & 0xff) as u32;
                }
                LHU_FUNCT3 => {
                    println!("LHU");
                    self.regs[rd] = (self
                        .bus
                        .load(
                            Cpu::alu_add(
                                self.regs[rs1],
                                Cpu::sign_extend((instruction & 0xfff00000) >> 20),
                            ) as usize,
                            16,
                        )
                        .unwrap()
                        & 0xffff) as u32
                }
                _ => {
                    todo!();
                }
            },
            STORE_OPCODE => {
                let imm11to5 = (instruction >> 25) & 0x7f;
                let imm4to0 = (instruction >> 7) & 0x1f;
                let imm = (imm11to5 << 5) | (imm4to0);
                println!(
                    "imm11to5: {:#32b} imm4to0: {:#32b}, imm: {:#32b}",
                    imm11to5, imm4to0, imm
                );
                let addr: usize = (Cpu::alu_add(self.regs[rs1], Cpu::sign_extend(imm)))
                    .try_into()
                    .unwrap();
                match funct3 {
                    SB_FUNCT3 => {
                        println!("SB");
                        self.bus
                            .store(addr, 8, ((self.regs[rs2]) & 0xff).try_into().unwrap())
                            .unwrap();
                    }
                    SH_FUNCT3 => {
                        println!("SH");
                        self.bus
                            .store(addr, 16, ((self.regs[rs2]) & 0xffff).try_into().unwrap())
                            .unwrap();
                    }
                    SW_FUNCT3 => {
                        println!("SW");
                        self.bus
                            .store(addr, 32, (self.regs[rs2]).try_into().unwrap())
                            .unwrap();
                    }
                    _ => {
                        todo!();
                    }
                }
            }
            _ => {
                todo!();
            }
        }
    }
}
