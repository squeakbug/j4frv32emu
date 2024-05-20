use std::ops::Shl;
use std::ops::Shr;

use crate::errors::ProcessorError;
use crate::opcodes::*;
use crate::decode::*;

const NREGS: usize = 32;

enum Inst {
    Inst16(u16),
    Inst32(u32),
    Inst48(u64),
    Inst64(u64),
}

pub struct Processor {
    regs: [u64; NREGS],

    pc: u64,
    mem: Vec<u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            regs: [0; NREGS],
            pc: 0,
            mem: vec![0u8; 0x100000],
        }
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    pub fn load(&mut self, data: Vec<u8>) {
        self.mem = data;
    }

    pub fn load_to(&mut self, data: Vec<u8>, addr: u64) {
        let addr = addr - 0x8000_0000;
        let mut tmp = Vec::from(&self.mem[..addr as usize]);
        tmp.extend(data.as_slice());
        tmp.extend(&self.mem[addr as usize + data.len()..]);
        self.mem = tmp;
    }

    fn fetch(&self) -> Inst {
        let index = self.pc as usize - 0x8000_0000;
        let inst16 = ((self.mem[index] as u16) << 0) 
                   | ((self.mem[index + 1] as u16) << 8);
        if inst16 & 0x3 < 0x3 {
            return Inst::Inst16(inst16);
        }
        let inst32 = (inst16 as u32)
            | ((self.mem[index + 2] as u32) << 16)
            | ((self.mem[index + 3] as u32) << 24);
        if inst32 & 0x1f < 0x1f {
            return Inst::Inst32(inst32);
        }
        let inst48 = (inst32 as u64)
            | ((self.mem[index + 4] as u64) << 32)
            | ((self.mem[index + 5] as u64) << 40);
        if inst48 & 0x3f < 0x3f {
            return Inst::Inst48(inst48);
        }
        let inst64 = (inst48 as u64)
            | ((self.mem[index + 6] as u64) << 48)
            | ((self.mem[index + 7] as u64) << 56);
        if inst64 ^ 0x7f == 0 {
            return Inst::Inst64(inst64);
        }
        panic!("Not supported opcode");
    }

    fn execute_16(&mut self, inst: u16) -> Result<(), ProcessorError> {
        let opcode = inst & 0x3;
        let rd_rs1 = (((inst) >> 7) & 0xf) as usize;
        let rs2 = ((inst >> 2) & 0x1f) as usize;
        let funct3 = ((inst >> 13) & 0x7) as usize;

        match opcode {
            0x1 => {
                match funct3 {
                    0x1 => {
                        // addi
                        let imm = ((inst & 0x007c) as i32 as i64 >> 2) as u64;
                        self.regs[rd_rs1] = self.regs[rd_rs1].wrapping_add(imm);
                    }
                    0x4 => {
                        // add
                        self.regs[rd_rs1] = self.regs[rd_rs1].wrapping_add(self.regs[rs2]);
                    }
                    _ => return Err(ProcessorError::NotYetImplemented),
                }
            }
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn exec_ADD(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)].wrapping_add(self.regs[rs2(inst)]);
    }

    fn exec_SUB(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)].wrapping_sub(self.regs[rs2(inst)]);
    }

    fn exec_SLL(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)].shl(self.regs[rs2(inst)]);
    }

    fn exec_SLT(&mut self, inst: u32) {
        self.regs[rd(inst)] = if self.regs[rs1(inst)] < self.regs[rs2(inst)] { 0x1 } else { 0x0 };
    }

    fn exec_SLTU(&mut self, inst: u32) {
        self.regs[rd(inst)] = if self.regs[rs1(inst)] < self.regs[rs2(inst)] { 0x1 } else { 0x0 };
    }

    fn exec_XOR(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)] ^ self.regs[rs2(inst)];
    }

    fn exec_SRL(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)].shr(self.regs[rs2(inst)]);
    }

    fn exec_SRA(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)].shr(self.regs[rs2(inst)]);
    }

    fn exec_AND(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)] & self.regs[rs2(inst)];
    }

    fn exec_OR(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)] | self.regs[rs2(inst)];
    }

    fn exec_r_type(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let funct7 = funct7(inst);
        let funct3 = funct3(inst);
        match funct7 {
            ADD_FUNCT3 => match funct3 {
                ADD => self.exec_ADD(inst),
                SUB => self.exec_SUB(inst),
                _ => return Err(ProcessorError::NotYetImplemented),
            },
            SLL => self.exec_SLL(inst),
            SLT => self.exec_SLT(inst),
            SLTU => self.exec_SLTU(inst),
            XOR => self.exec_XOR(inst),
            SRL_FUNCT3 => match funct3 {
                SRL => self.exec_SRL(inst),
                SRA => self.exec_SRA(inst),   
                _ => return Err(ProcessorError::NotYetImplemented),
            }
            OR  => self.exec_OR(inst),
            AND => self.exec_AND(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn exec_ADDI(&mut self, inst: u32) {
        let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
        self.regs[rd(inst)] = self.regs[rs1(inst)].wrapping_add(imm);
    }

    fn exec_SLLI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_SLTI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_SLTIU(&mut self, inst: u32) {
        todo!()
    }

    fn exec_XORI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_SRLI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_SRAI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_ANDI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_ORI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_i_type(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let funct7 = funct7(inst);
        let funct3 = funct3(inst);
        match funct7 {
            ADDI => self.exec_ADDI(inst),
            SLLI => self.exec_SLLI(inst),
            SLTI => self.exec_SLTI(inst),
            SLTIU => self.exec_SLTIU(inst),
            XORI => self.exec_XORI(inst),
            SRI_FUNCT3 => match funct3 {
                SRLI => self.exec_SRLI(inst),
                SRAI => self.exec_SRAI(inst),   
                _ => return Err(ProcessorError::NotYetImplemented),
            }
            ORI  => self.exec_ORI(inst),
            ANDI => self.exec_ANDI(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn execute_32(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let opcode = inst & 0x7f;
        match opcode {
            R_TYPE => self.exec_r_type(inst),
            I_TYPE => self.exec_i_type(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
    }

    fn execute_48(&mut self, _: u64) -> Result<(), ProcessorError> {
        todo!()
    }

    fn execute_64(&mut self, inst: u64) -> Result<(), ProcessorError> {
        let opcode = inst & 0x7f;
        let rd = (((inst) >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;

        match opcode {
            0x13 => {
                // addi
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn execute(&mut self, inst: Inst) -> Result<(), ProcessorError> {
        match inst {
            Inst::Inst16(inner) => self.execute_16(inner),
            Inst::Inst32(inner) => self.execute_32(inner),
            Inst::Inst48(inner) => self.execute_48(inner),
            Inst::Inst64(inner) => self.execute_64(inner),
        }
    }

    pub fn dump(&self) -> String {
        let abi = [
            "zero", "ra",  "sp",  "gp",
              "tp", "t0",  "t1",  "t2",
              "s0", "s1",  "a0",  "a1",
              "a2", "a3",  "a4",  "a5",
              "a6", "a7",  "s2",  "s3",
              "s4", "s5",  "s6",  "s7",
              "s8", "s9", "s10", "s11",
              "t3", "t4",  "t5",  "t6",
        ];

        let mut out = String::from("");
        for i in 0..8 {
            out += &format!("{:4}: {:<8x}", abi[i], self.regs[i]);
            out += &format!("{:2}: {:<8x}", abi[i + 8], self.regs[i + 8]);
            out += &format!("{:2}: {:<8x}", abi[i + 16], self.regs[i + 16]);
            out += &format!("{:3}: {:<8x}\n", abi[i + 24], self.regs[i + 24]);
        }
        out
    }

    pub fn tick(&mut self) -> Result<(), ProcessorError> {
        if self.pc as usize - 0x8000_0000 >= self.mem.len() {
            return Err(ProcessorError::BufferOverflow);
        }
        let inst = self.fetch();
        self.execute(inst)?;
        self.pc = self.pc + 4;
        Ok(())
    }
}
