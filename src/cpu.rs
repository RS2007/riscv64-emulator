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
const ADD_SUB_FUNCT3: u32 = 0b000;
const SLL_FUNCT3: u32 = 0b001;
const SLT_FUNCT3: u32 = 0b010;
const SLTU_FUNCT3: u32 = 0b011;
const XOR_FUNCT3: u32 = 0b100;
const SRL_SRA_FUNCT3: u32 = 0b101;
const SRL_FUNCT7: u32 = 0b0000000;
const SRA_FUNCT7: u32 = 0b0100000;
const OR_FUNCT3: u32 = 0b110;
const AND_FUNCT3: u32 = 0b111;
const ADDI_FUNCT3: u32 = 0b000;
const SLTI_FUNCT3: u32 = 0b010;
const SLTIU_FUNCT3: u32 = 0b011;
const XORI_FUNCT3: u32 = 0b100;
const ORI_FUNCT3: u32 = 0b110;
const ANDI_FUNCT3: u32 = 0b111;
const SLLI_FUNCT3: u32 = 0b001;
const SRLI_SRAI_FUNCT3: u32 = 0b101;
const SRLI_FUNCT7: u32 = 0b0000000;
const SRAI_FUNCT7: u32 = 0b0100000;
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
const BRANCH_OPCODE: u32 = 0b1100011;
const BEQ_FUNCT3: u32 = 0b000;
const BNEQ_FUNCT3: u32 = 0b001;
const BLT_FUNCT3: u32 = 0b100;
const BGE_FUNCT3: u32 = 0b101;
const BLTU_FUNCT3: u32 = 0b110;
const BGEU_FUNCT3: u32 = 0b111;
const JALR_OPCODE: u32 = 0b1100111;
const JAL_OPCODE: u32 = 0b1101111;
const AUIPC_OPCODE: u32 = 0b0010111;
const LUI_OPCODE: u32 = 0b0110111;

impl Cpu {
    pub fn new(dram: Dram) -> Self {
        return Cpu {
            pc: DRAM_BASE,
            regs: vec![0; 32],
            bus: bus::Bus::new(dram),
        };
    }

    pub fn print_registers(&self) {
        println!("Printing regs");
        println!("---------------------");
        self.regs.iter().enumerate().for_each(|(index, reg)| {
            println!("{index}: {:#x}", reg);
        });
        println!("---------------------");
        println!("Register ends");
    }

    pub fn print_mem_around_interest(&self, addr: usize) {
        println!("Printing mem");
        println!("-----------------------");
        for i in (addr - 40)..=addr {
            println!("{i} {}", self.bus.load(i, 8).unwrap());
        }
        println!("-----------------------");
        println!("Mem ends");
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
    fn sign_extend(imm: u32, instruction: u32) -> u32 {
        let opcode = instruction & 0x7f;
        println!("opcode: {:#x}", opcode);
        return match opcode {
            LUI_OPCODE | AUIPC_OPCODE => imm,
            JAL_OPCODE => {
                let top_bit = (imm >> 20) & 0x1;
                return match top_bit {
                    0 => imm,
                    1 => 0xffe00000 | imm,
                    _ => {
                        assert!(false, "Should'nt hit this");
                        return 0;
                    }
                };
            }
            BRANCH_OPCODE => {
                let top_bit = (imm >> 12) & 0x1;
                return match top_bit {
                    0 => imm,
                    1 => 0xffffe000 | imm,
                    _ => {
                        assert!(false, "Should'nt hit this");
                        return 0;
                    }
                };
            }
            LOAD_OPCODE | JALR_OPCODE | STORE_OPCODE | ALUI_OPCODE => {
                let top_bit = (imm >> 11) & 0x1;
                return match top_bit {
                    0 => imm,
                    1 => imm | 0xffffff00,
                    _ => {
                        assert!(false, "Should'nt hit this");
                        return 0;
                    }
                };
            }
            _ => {
                assert!(false, "Should'nt hit this");
                return 0;
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
            ALU_OPCODE => {
                match funct3 {
                    ADD_SUB_FUNCT3 => match funct7 {
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
                    SLL_FUNCT3 => {
                        println!("SLL: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                        self.regs[rd] = self.regs[rs1] << (self.regs[rs2] & 0x1f);
                    }
                    SLT_FUNCT3 => {
                        println!("SLT: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                        self.regs[rd] = match (self.regs[rs1] as i32) < (self.regs[rs2] as i32) {
                            true => 1,
                            false => 0,
                        };
                    }
                    SLTU_FUNCT3 => {
                        println!("SLTU: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                        self.regs[rd] = match (self.regs[rs1] as u32) < (self.regs[rs2] as u32) {
                            true => 1,
                            false => 0,
                        };
                    }
                    XOR_FUNCT3 => {
                        println!("SLTU: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                        self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
                    }
                    SRL_SRA_FUNCT3 => match funct7 {
                        SRL_FUNCT7 => {
                            println!("SRL: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                            self.regs[rd] =
                                (self.regs[rs1] as u32) >> ((self.regs[rs2] & 0x1f) as u32);
                        }
                        SRA_FUNCT7 => {
                            println!("SRA: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                            self.regs[rd] =
                                (self.regs[rs1] as i32).wrapping_shr(self.regs[rs2]) as u32;
                        }
                        _ => {
                            assert!(false, "Should'nt hit this");
                        }
                    },
                    OR_FUNCT3 => {
                        println!("OR: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                        self.regs[rd] = (self.regs[rs1]) | (self.regs[rs2]);
                    }
                    AND_FUNCT3 => {
                        println!("AND: rs1={:?} rs2={:?} rd={:?}", rs1, rs2, rd);
                        self.regs[rd] = (self.regs[rs1]) & (self.regs[rs2]);
                    }
                    _ => {
                        todo!();
                    }
                }
                self.pc += 4;
            }
            ALUI_OPCODE => {
                match funct3 {
                    ADDI_FUNCT3 => {
                        let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                        println!("Add: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                        self.regs[rd] = Cpu::alu_add(self.regs[rs1], imm);
                    }
                    SLTI_FUNCT3 => {
                        let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                        println!("Slti: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                        self.regs[rd] = match (self.regs[rs1] as i32) < (imm as i32) {
                            true => 1,
                            false => 0,
                        }
                    }
                    SLTIU_FUNCT3 => {
                        let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                        println!("Sltiu: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                        self.regs[rd] = match self.regs[rs1] < imm {
                            true => 1,
                            false => 0,
                        };
                    }
                    XORI_FUNCT3 => {
                        let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                        println!("Xori: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                        self.regs[rd] = self.regs[rs1] | imm;
                    }
                    ORI_FUNCT3 => {
                        let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                        println!("Or: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                        self.regs[rd] = self.regs[rs1] | imm;
                    }
                    ANDI_FUNCT3 => {
                        let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                        println!("And: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                        self.regs[rd] = self.regs[rs1] & imm;
                    }
                    SLLI_FUNCT3 => {
                        let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                        println!("Slli: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                        self.regs[rd] = self.regs[rs1] << imm;
                    }
                    SRLI_SRAI_FUNCT3 => match funct7 {
                        SRLI_FUNCT7 => {
                            let imm =
                                Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                            println!("Srli: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                            self.regs[rd] = (self.regs[rs1] as u32) >> ((imm & 0x1f) as u32);
                        }
                        SRAI_FUNCT7 => {
                            let imm =
                                Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                            println!("Srai: rs1={:?} imm={:?} rd={:?}", rs1, imm, rd);
                            self.regs[rd] =
                                ((self.regs[rs1] as i32) >> ((imm & 0x1f) as u32)) as u32;
                        }
                        _ => {
                            todo!();
                        }
                    },
                    _ => {
                        assert!(false, "Should'nt hit this");
                    }
                }
                self.pc += 4;
            }
            LOAD_OPCODE => {
                match funct3 {
                    LB_FUNCT3 => {
                        println!("Load Byte");
                        self.regs[rd] = Cpu::sign_extend(
                            (self
                                .bus
                                .load(
                                    Cpu::alu_add(
                                        self.regs[rs1],
                                        Cpu::sign_extend(
                                            (instruction & 0xfff00000) >> 20,
                                            instruction,
                                        ),
                                    ) as usize,
                                    8,
                                )
                                .unwrap()
                                & 0xff) as u32,
                            instruction,
                        )
                    }
                    LH_FUNCT3 => {
                        println!("Load Half Word");
                        self.regs[rd] = Cpu::sign_extend(
                            (self
                                .bus
                                .load(
                                    Cpu::alu_add(
                                        self.regs[rs1],
                                        Cpu::sign_extend(
                                            (instruction & 0xfff00000) >> 20,
                                            instruction,
                                        ),
                                    ) as usize,
                                    16,
                                )
                                .unwrap()
                                & 0xffff) as u32,
                            instruction,
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
                                    Cpu::sign_extend((instruction >> 20) & 0xfff, instruction)
                                ) as usize,
                                32,
                            ))
                            .unwrap()
                        );
                        self.regs[rd] = (self
                            .bus
                            .load(
                                Cpu::alu_add(
                                    self.regs[rs1],
                                    Cpu::sign_extend((instruction >> 20) & 0xfff, instruction),
                                ) as usize,
                                32,
                            )
                            .unwrap()
                            & 0xffffffff) as u32
                    }
                    LBU_FUNCT3 => {
                        println!("LBU");
                        self.regs[rd] = (self
                            .bus
                            .load(
                                Cpu::alu_add(
                                    self.regs[rs1],
                                    Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction),
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
                                    Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction),
                                ) as usize,
                                16,
                            )
                            .unwrap()
                            & 0xffff) as u32
                    }
                    _ => {
                        todo!();
                    }
                }
                self.pc += 4;
            }
            STORE_OPCODE => {
                let imm11to5 = (instruction >> 25) & 0x7f;
                let imm4to0 = (instruction >> 7) & 0x1f;
                let imm = (imm11to5 << 5) | (imm4to0);
                println!(
                    "imm11to5: {:#32b} imm4to0: {:#32b}, imm: {:#32b}",
                    imm11to5, imm4to0, imm
                );
                let addr: usize =
                    (Cpu::alu_add(self.regs[rs1], Cpu::sign_extend(imm, instruction)))
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
                self.pc += 4;
            }
            BRANCH_OPCODE => {
                let imm11to5: u32 = (instruction >> 25) & 0x7f;
                let imm12: u32 = (instruction >> 31) & 0x1;
                let imm10to5: u32 = imm11to5 & 0x3f;
                let imm4to1: u32 = ((rd >> 1) & 0xf) as u32;
                let imm11: u32 = (rd & 0x1) as u32;
                let imm_branch = (imm12 << 12) | (imm11 << 11) | (imm10to5 << 5) | (imm4to1 << 1);
                match funct3 {
                    BEQ_FUNCT3 => {
                        if self.regs[rs1] == self.regs[rs2] {
                            self.pc = Cpu::alu_add(
                                self.pc as u32,
                                Cpu::sign_extend(imm_branch, instruction),
                            ) as usize;
                        } else {
                            self.pc += 4;
                        }
                        println!("BEQ: pc going to {:#x}", self.pc);
                    }
                    BNEQ_FUNCT3 => {
                        if self.regs[rs1] != self.regs[rs2] {
                            self.pc = Cpu::alu_add(
                                self.pc as u32,
                                Cpu::sign_extend(imm_branch, instruction),
                            ) as usize;
                        } else {
                            self.pc += 4;
                        }
                        println!("BNEQ: pc going to {:#x}", self.pc);
                    }
                    BLT_FUNCT3 => {
                        if (self.regs[rs1] as i32) < (self.regs[rs2] as i32) {
                            self.pc = Cpu::alu_add(
                                self.pc as u32,
                                Cpu::sign_extend(imm_branch, instruction),
                            ) as usize;
                        } else {
                            self.pc += 4;
                        }
                        println!("BLT: pc going to {:#x}", self.pc);
                    }
                    BGE_FUNCT3 => {
                        if (self.regs[rs1] as i32) >= (self.regs[rs2] as i32) {
                            self.pc = Cpu::alu_add(
                                self.pc as u32,
                                Cpu::sign_extend(imm_branch, instruction),
                            ) as usize;
                        } else {
                            self.pc += 4;
                        }
                        println!("BGE: pc going to {:#x}", self.pc);
                    }
                    BLTU_FUNCT3 => {
                        if self.regs[rs1] < self.regs[rs2] {
                            self.pc = Cpu::alu_add(
                                self.pc as u32,
                                Cpu::sign_extend(imm_branch, instruction),
                            ) as usize;
                        } else {
                            self.pc += 4;
                        }
                        println!("BLTU: pc going to {:#x}", self.pc);
                    }
                    BGEU_FUNCT3 => {
                        if self.regs[rs1] >= self.regs[rs2] {
                            self.pc = Cpu::alu_add(
                                self.pc as u32,
                                Cpu::sign_extend(imm_branch, instruction),
                            ) as usize;
                        } else {
                            self.pc += 4;
                        }
                        println!("BGEU: pc going to {:#x}", self.pc);
                    }
                    _ => {
                        todo!();
                    }
                }
            }
            JALR_OPCODE => {
                // t =pc+4; pc=(x[rs1]+sext(offset))&∼1; x[rd]=t
                let imm = Cpu::sign_extend((instruction & 0xfff00000) >> 20, instruction);
                let old_pc = self.pc;
                self.pc = Cpu::alu_add(
                    self.regs[rs1],
                    Cpu::sign_extend(imm, instruction) & 0xfffffffe,
                ) as usize;

                self.regs[rd] = (old_pc + 4) as u32;
                println!("JALR: pc going to {:#x}", self.pc);
            }
            JAL_OPCODE => {
                let imm20 = (instruction >> 31) & 0x1;
                let imm10to1 = (instruction >> 21) & 0x3ff;
                let imm11 = (instruction >> 20) & 0x1;
                let imm19to12 = (instruction >> 12) & 0xff;
                let imm_jal = (imm20 << 20) | (imm19to12 << 12) | (imm11 << 11) | (imm10to1 << 1);
                let imm = Cpu::sign_extend(imm_jal, instruction);
                println!("jalr imm:{:#x}", imm);
                self.regs[rd] = (self.pc + 4) as u32;
                self.pc = Cpu::alu_add(self.pc as u32, imm) as usize;
                println!("JAL: pc going to {:#x}", self.pc);
            }
            AUIPC_OPCODE => {
                let imm31to12 = (instruction >> 12) & 0x7ffff;
                self.regs[rd] = Cpu::alu_add(
                    self.pc as u32,
                    Cpu::sign_extend(imm31to12 << 12, instruction),
                );
                self.pc += 4;
                println!("AUIPC");
            }
            LUI_OPCODE => {
                let imm31to12 = (instruction >> 12) & 0x7ffff;
                self.regs[rd] = Cpu::sign_extend(imm31to12 << 12, instruction);
                self.pc += 4;
                println!("LUI");
            }
            _ => {
                todo!();
            }
        }
    }
}
