#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::ops::Shl;
use std::ops::Shr;

use crate::errors::*;
use crate::opcodes::*;
use crate::decode::*;
use crate::system_bus::*;

const NREGS: usize = 32;

#[derive(Debug, Clone, Copy)]
enum Inst {
    Inst16(u16),
    Inst32(u32),
    Inst48(u64),
    Inst64(u64),
}

pub struct Processor {
    regs: [u64; NREGS],

    pc: u64,
    system_bus: SystemBus,

    mvendorid: u64,
    marchid: u64,
    mimpid: u64,
    mhartid: u64,

    mstatus: u64,
    medeleg: u64,
    mideleg:  u64,
    mie: u64,
    mtvec: u64,

    mscratch: u64,
    mepc: u64,
    mcause: u64,
    mtval: u64,
    mip: u64,
}

impl Processor {
    pub fn new(system_bus: SystemBus) -> Self {
        Processor {
            regs: [0; NREGS],
            pc: 0,
            system_bus,

            mvendorid: 0,
            marchid: 0,
            mimpid: 0,
            mhartid: 0,

            mstatus: 0,
            medeleg: 0,
            mideleg:  0,
            mie: 0,
            mtvec: 0,

            mscratch: 0,
            mepc: 0,
            mcause: 0,
            mtval: 0,
            mip: 0,
        }
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    // TODO: Определение типа инструкции за один fetch
    // TODO: Заворачивание ошибки доступа к шине в ошибку процессора
    fn fetch(&self) -> Result<Inst, ProcessorError> {
        let inst = self.system_bus.load(self.pc, 64).map_err(|err| {
            ProcessorError::FetchError
        })?;

        if inst & 0x3 < 0x3 {
            return Ok(Inst::Inst16(inst as u16));
        }
        if inst & 0x1f < 0x1f {
            return Ok(Inst::Inst32(inst as u32));
        }
        if inst & 0x3f < 0x3f {
            return Ok(Inst::Inst48(inst as u64));
        }
        if inst ^ 0x7f == 0 {
            return Ok(Inst::Inst64(inst as u64));
        }
        panic!("Not supported opcode");
    }
}

impl Processor {
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
}

impl Processor {
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
        self.regs[rd(inst)] = self.regs[rs1(inst)] << shamt(inst);
    }

    fn exec_SLTI(&mut self, inst: u32) {
        if (self.regs[rs1(inst)] as i64) < (imm_I(inst) as i64) {
            self.regs[rd(inst)] = 1;
        } else {
            self.regs[rd(inst)] = 0;
        }
    }

    fn exec_SLTIU(&mut self, inst: u32) {
        if self.regs[rs1(inst)] < imm_I(inst) {
            self.regs[rd(inst)] = 1;
        } else {
            self.regs[rd(inst)] = 0;
        }
    }

    fn exec_XORI(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)] ^ imm_I(inst);
    }

    fn exec_SRLI(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)] >> imm_I(inst);
    }

    fn exec_SRAI(&mut self, inst: u32) {
        self.regs[rd(inst)] = ((self.regs[rs1(inst)] as i64) >> imm_I(inst)) as u64;
    }

    fn exec_ANDI(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)] & imm_I(inst);
    }

    fn exec_ORI(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.regs[rs1(inst)] | imm_I(inst);
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

    fn exec_LUI(&mut self, inst: u32) {
        self.regs[rd(inst)] = imm_U(inst);
    }

    fn exec_AUIPC(&mut self, inst: u32) {
        self.regs[rd(inst)] = imm_U(inst) + self.pc;
    }

    fn exec_u_type(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let opcode = op(inst);
        match opcode {
            LUI => self.exec_LUI(inst),
            AUIPC => self.exec_AUIPC(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn exec_JAL(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.pc + 4;
        self.pc = self.pc + imm_J(inst);
    }

    fn exec_JALR(&mut self, inst: u32) {
        self.regs[rd(inst)] = self.pc + 4;
        self.pc = self.regs[rs1(inst)] + imm_I(inst);
    }

    fn exec_j_type(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let opcode = op(inst);
        match opcode {
            JAL => self.exec_JAL(inst),
            JALR => self.exec_JALR(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn exec_BEQ(&mut self, inst: u32) {
        if self.regs[rs1(inst)] == self.regs[rs2(inst)] {
            self.pc = self.pc + imm_B(inst);
        }
    }

    fn exec_BNE(&mut self, inst: u32) {
        if self.regs[rs1(inst)] != self.regs[rs2(inst)] {
            self.pc = self.pc + imm_B(inst);
        }
    }

    fn exec_BLT(&mut self, inst: u32) {
        if (self.regs[rs1(inst)] as i64) < (self.regs[rs2(inst)] as i64) {
            self.pc = self.pc + imm_B(inst);
        }
    }

    fn exec_BGE(&mut self, inst: u32) {
        if (self.regs[rs1(inst)] as i64) >= (self.regs[rs2(inst)] as i64) {
            self.pc = self.pc + imm_B(inst);
        }
    }

    fn exec_BLTU(&mut self, inst: u32) {
        if self.regs[rs1(inst)] < self.regs[rs2(inst)] {
            self.pc = self.pc + imm_B(inst);
        }
    }

    fn exec_BGEU(&mut self, inst: u32) {
        if self.regs[rs1(inst)] >= self.regs[rs2(inst)] {
            self.pc = self.pc + imm_B(inst);
        }
    }

    fn exec_b_type(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let opcode = op(inst);
        let funct3 = funct3(inst);
        match funct3 {
            BEQ  => self.exec_BEQ(inst),
            BNE  => self.exec_BNE(inst),
            BLT  => self.exec_BLT(inst),
            BGE  => self.exec_BGE(inst),
            BLTU => self.exec_BLTU(inst),
            BGEU => self.exec_BGEU(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn exec_LB(&mut self, inst: u32) {
        const SZ: usize = 8;
        self.regs[rd(inst)] = self.regs[rd(inst)] & (!0xff) 
            | self.system_bus.load(rs1(inst) as u64, SZ).expect("Read bus error");
    }

    fn exec_LH(&mut self, inst: u32) {
        const SZ: usize = 16;
        self.regs[rd(inst)] = self.regs[rd(inst)] & (!0xffff)
            | self.system_bus.load(rs1(inst) as u64, SZ).expect("Read bus error");
    }

    fn exec_LW(&mut self, inst: u32) {
        const SZ: usize = 32;
        self.regs[rd(inst)] = self.regs[rd(inst)] & (!0xffff_ffff)
            | self.system_bus.load(rs1(inst) as u64, SZ).expect("Read bus error");
    }

    fn exec_LD(&mut self, inst: u32) {
        const SZ: usize = 64;
        self.regs[rd(inst)] = self.regs[rd(inst)]
            | self.system_bus.load(rs1(inst) as u64, SZ).expect("Read bus error");
    }

    fn exec_LBU(&mut self, inst: u32) {
        const SZ: usize = 8;
        self.regs[rd(inst)] = self.regs[rd(inst)] & (!0xff) 
            | self.system_bus.load(rs1(inst) as u64, SZ).expect("Read bus error");
    }

    fn exec_LHU(&mut self, inst: u32) {
        const SZ: usize = 16;
        self.regs[rd(inst)] = self.regs[rd(inst)] & (!0xffff)
            | self.system_bus.load(rs1(inst) as u64, SZ).expect("Read bus error");
    }

    fn exec_LWU(&mut self, inst: u32) {
        const SZ: usize = 32;
        self.regs[rd(inst)] = self.regs[rd(inst)] & (!0xffff_ffff)
            | self.system_bus.load(rs1(inst) as u64, SZ).expect("Read bus error");
    }

    fn exec_load(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let funct3 = funct3(inst);
        match funct3 {
            LB => self.exec_LB(inst),
            LH => self.exec_LH(inst),
            LW => self.exec_LW(inst),
            LD => self.exec_LD(inst),
            LBU => self.exec_LBU(inst),
            LHU => self.exec_LHU(inst),
            LWU => self.exec_LWU(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn exec_SB(&mut self, inst: u32) {
        const SZ: usize = 8;
        let offset = imm_S(inst);
        let store_addr = offset + self.regs[rs1(inst)];
        let data = rs2(inst);
        self.system_bus.store(data as u64, store_addr, SZ).expect("Read bus error");
    }

    fn exec_SH(&mut self, inst: u32) {
        const SZ: usize = 16;
        let offset = imm_S(inst);
        let store_addr = offset + self.regs[rs1(inst)];
        let data = rs2(inst);
        self.system_bus.store(data as u64, store_addr, SZ).expect("Read bus error");
    }

    fn exec_SW(&mut self, inst: u32) {
        const SZ: usize = 32;
        let offset = imm_S(inst);
        let store_addr = offset + self.regs[rs1(inst)];
        let data = rs2(inst);
        self.system_bus.store(data as u64, store_addr, SZ).expect("Read bus error");
    }

    fn exec_SD(&mut self, inst: u32) {
        const SZ: usize = 64;
        let offset = imm_S(inst);
        let store_addr = offset + self.regs[rs1(inst)];
        let data = rs2(inst);
        self.system_bus.store(data as u64, store_addr, SZ).expect("Read bus error");
    }

    fn exec_store(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let funct3 = funct3(inst);
        match funct3 {
            SB => self.exec_SB(inst),
            SH => self.exec_SH(inst),
            SW => self.exec_SW(inst),
            SD => self.exec_SD(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn exec_ECALL(&mut self, inst: u32) {
        todo!()
    }

    fn exec_EBREAK(&mut self, inst: u32) {
        todo!()
    }

    fn exec_SRET(&mut self, inst: u32) {
        todo!()
    }

    fn exec_MRET(&mut self, inst: u32) {
        todo!()
    }

    fn exec_CSRRW(&mut self, inst: u32) {
        todo!()
    }

    fn exec_CSRRS(&mut self, inst: u32) {
        todo!()
    }

    fn exec_CSRRC(&mut self, inst: u32) {
        todo!()
    }

    fn exec_CSRRWI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_CSRRSI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_CSRRCI(&mut self, inst: u32) {
        todo!()
    }

    fn exec_system(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let funct3 = funct3(inst);
        let funct7 = funct7(inst);
        match funct3 {
            ECALLBREAK => match funct7 {
                ECALL  => self.exec_ECALL(inst),
                EBREAK => self.exec_EBREAK(inst),
                SRET   => self.exec_SRET(inst),
                MRET   => self.exec_MRET(inst),
                _ => return Err(ProcessorError::NotYetImplemented),
            },
            CSRRW => self.exec_CSRRW(inst),
            CSRRS => self.exec_CSRRS(inst),
            CSRRC => self.exec_CSRRC(inst),
            CSRRWI => self.exec_CSRRWI(inst),
            CSRRSI => self.exec_CSRRSI(inst),
            CSRRCI => self.exec_CSRRCI(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn exec_FENCE(&mut self, inst: u32) {
        todo!()
    }

    fn exec_FENCE_I(&mut self, inst: u32) {
        todo!()
    }

    fn exec_fence(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let funct3 = funct3(inst);
        match funct3 {
            FENCE_FUNCT3 => self.exec_FENCE(inst),
            FENCE_I_FUNCT3 => self.exec_FENCE_I(inst),
            _ => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }

    fn execute_32(&mut self, inst: u32) -> Result<(), ProcessorError> {
        let opcode = op(inst);
        match opcode {
            LUI    => self.exec_LUI(inst),
            AUIPC  => self.exec_AUIPC(inst),
            R_TYPE => return self.exec_r_type(inst),
            I_TYPE => return self.exec_i_type(inst),
            JAL    => self.exec_JAL(inst),
            JALR   => self.exec_JALR(inst),
            B_TYPE => return self.exec_b_type(inst),
            LOAD   => return self.exec_load(inst),
            S_TYPE => return self.exec_store(inst),
            SYSTEM => return self.exec_system(inst),
            FENCE  => return self.exec_fence(inst),
            _      => return Err(ProcessorError::NotYetImplemented),
        }
        Ok(())
    }
}

impl Processor {
    fn execute_48(&mut self, _: u64) -> Result<(), ProcessorError> {
        todo!()
    }
}

impl Processor {
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
}

impl Processor {
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
        let inst = self.fetch()?;
        self.execute(inst)?;
        self.pc = self.pc + 4;
        Ok(())
    }
}
