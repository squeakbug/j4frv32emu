pub fn rd(inst: u32) -> usize {
    // rd in bits 11..7
    return ((inst >> 7) & 0x1f) as usize;
}

pub fn rs1(inst: u32) -> usize {
    // rs1 in bits 19..15
    return ((inst >> 15) & 0x1f) as usize;
}

pub fn rs2(inst: u32) -> usize {
    return ((inst >> 20) & 0x1f) as usize;   // rs2 in bits 24..20
}

pub fn imm_I(inst: u32) -> u64 {
    // imm[11:0] = inst[31:20]
    return ((inst & 0xfff00000) >> 20) as u64; // right shift as signed?
}

pub fn imm_S(inst: u32) -> u64 {
    // imm[11:5] = inst[31:25], imm[4:0] = inst[11:7]
    return (((inst & 0xfe000000) >> 20)
         | ((inst >> 7) & 0x1f)) as u64;
}

pub fn imm_B(inst: u32) -> u64 {
    // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
    return (((inst & 0x80000000) >> 19)
        | ((inst & 0x80) << 4) // imm[11]
        | ((inst >> 20) & 0x7e0) // imm[10:5]
        | ((inst >> 7) & 0x1e)) as u64; // imm[4:1]
}

pub fn imm_U(inst: u32) -> u64 {
    // imm[31:12] = inst[31:12]
    return (inst & 0xfffff000) as u64;
}

pub fn imm_J(inst: u32) -> u64 {
    // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
    return (((inst & 0x80000000) >> 11)
        | (inst & 0xff000) // imm[19:12]
        | ((inst >> 9) & 0x800) // imm[11]
        | ((inst >> 20) & 0x7fe)) as u64; // imm[10:1]
}

pub fn csr(inst: u32) -> u64 {
    x(inst as u64, 20, 12)
}

pub fn funct3(inst: u32) -> u64 {
    x(inst as u64, 12, 3)
}

pub fn funct7(inst: u32) -> u64 {
    x(inst as u64, 7, 25)
}

pub fn op(inst: u32) -> u64 {
    x(inst as u64, 0, 7)
}

fn x(inst: u64, lo: usize, len: usize) -> u64 {
    (inst >> lo) & ((1u64 << len) - 1) 
}

fn xs(inst: u64, lo: usize, len: usize) -> u64 {
    ((inst as i64) << (64 - lo - len) >> (64 - len)) as u64
}

fn imm_sign(inst: u64) -> u64 {
    xs(inst, 31, 1)
}
