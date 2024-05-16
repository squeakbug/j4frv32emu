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

    halt: bool,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            regs: [0; NREGS],
            pc: 0,
            mem: vec![],

            halt: false,
        }
    }

    pub fn is_halted(&self) -> bool {
        return self.halt;
    }

    pub fn load(&mut self, data: Vec<u8>) {
        self.mem = data;
    }

    fn fetch(&self) -> Inst {
        let index = self.pc as usize;
        let inst16 = ((self.mem[index] as u16) << 0) 
                   | ((self.mem[index + 1] as u16) << 8);
        if inst16 & 0x3 != 0x3 {
            return Inst::Inst16(inst16);
        }
        let inst32 = (inst16 as u32)
            | ((self.mem[index + 2] as u32) << 16)
            | ((self.mem[index + 3] as u32) << 24);
        if inst32 & 0x1c != 0x1c {
            return Inst::Inst32(inst32);
        }
        let inst48 = (inst32 as u64)
            | ((self.mem[index + 4] as u64) << 32)
            | ((self.mem[index + 5] as u64) << 40);
        if inst48 & 0xe0 == 0xe0 {
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

    fn sign_extend(inst: u64) -> u64 {
        todo!()
    }

    fn execute_16(&mut self, inst: u16) {
        println!("inst16 = {:x}", inst);

        let opcode = inst & 0x3;
        let rd_rs1 = (((inst) >> 7) & 0xf) as usize;
        let rs2 = ((inst >> 2) & 0x1f) as usize;
        let funct3 = ((inst >> 13) & 0x7) as usize;

        match opcode {
            0x0 => {
                match funct3 {
                    _ => {
                        dbg!("not implemented yet");
                    }
                }
            }
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
                    _ => {
                        dbg!("not implemented yet");
                    }
                }
            }
            0x2 => {
                match funct3 {
                    _ => {
                        dbg!("not implemented yet");
                    }
                }
            }
            _ => {
                dbg!("not implemented yet");
            }
        }
    }

    fn execute_32(&mut self, inst: u32) {
        println!("inst32 = {:x}", inst);

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
            _ => {
                dbg!("not implemented yet");
            }
        }
    }

    fn execute_48(&mut self, inst: u64) {
        println!("inst48 = {:x}", inst);

        todo!()
    }

    fn execute_64(&mut self, inst: u64) {
        println!("inst64 = {:x}", inst);

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
            _ => {
                dbg!("not implemented yet");
            }
        }
    }

    fn execute(&mut self, inst: Inst) {
        match inst {
            Inst::Inst16(inner) => self.execute_16(inner),
            Inst::Inst32(inner) => self.execute_32(inner),
            Inst::Inst48(inner) => self.execute_48(inner),
            Inst::Inst64(inner) => self.execute_64(inner),
        }
    }

    pub fn dump(&self) -> String {
        let mut out = String::from("");
        for i in 0..NREGS {
            out += &(self.regs[i].to_string() + "\n");
        }
        out
    }

    pub fn tick(&mut self) {
        let inst = self.fetch();
        self.execute(inst);
        self.pc = self.pc + 4;
        if self.pc as usize >= self.mem.len() {
            self.halt = true;
        }
    }
}
