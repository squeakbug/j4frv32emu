#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

pub const LUI: u64 = 0x37; 
pub const AUIPC: u64 = 0x17; 

pub const JAL: u64 = 0x6f;
pub const JALR: u64 = 0x67;

pub const B_TYPE: u64 = 0x63;
pub const BEQ: u64 = 0x0;
pub const BNE: u64 = 0x1;
pub const BLT: u64 = 0x4;
pub const BGE: u64 = 0x5;
pub const BLTU: u64 = 0x6;
pub const BGEU: u64 = 0x7;

pub const LOAD: u64 = 0x03;
pub const LB: u64 = 0x0;
pub const LH: u64 = 0x1;
pub const LW: u64 = 0x2;
pub const LD: u64 = 0x3;
pub const LBU: u64 = 0x4;
pub const LHU: u64 = 0x5;
pub const LWU: u64 = 0x6;

pub const S_TYPE: u64 = 0x23;
pub const SB: u64 = 0x0;
pub const SH: u64 = 0x1;
pub const SW: u64 = 0x2;
pub const SD: u64 = 0x3;

pub const I_TYPE: u64 = 0x13;
    pub const ADDI: u64 = 0x0;
    pub const SLLI: u64 = 0x1;
    pub const SLTI: u64 = 0x2;
    pub const SLTIU: u64 = 0x3;
    pub const XORI: u64 = 0x4;
    pub const SRI_FUNCT3: u64 = 0x5;
        pub const SRLI: u64 = 0x00;
        pub const SRAI: u64 = 0x20;
    pub const ORI: u64 = 0x6;
    pub const ANDI: u64 = 0x7;

pub const R_TYPE: u64 = 0x33;
    pub const ADD_FUNCT3: u64 = 0x00;
        pub const ADD: u64 = 0x00;
        pub const SUB: u64 = 0x20;
    pub const SLL : u64 = 0x1;
    pub const SLT : u64 = 0x2;
    pub const SLTU: u64 = 0x3;
    pub const XOR : u64 = 0x4;
    pub const SRL_FUNCT3: u64 = 0x5;
        pub const SRL: u64 = 0x00;
        pub const SRA: u64 = 0x20;
    pub const OR : u64 = 0x6;
    pub const AND: u64 = 0x7;

pub const FENCE: u64 = 0x0f;
pub const FENCE_FUNCT3: u64 = 0x00;
pub const FENCE_I_FUNCT3: u64 = 0x01;

pub const I_TYPE_64: u64 = 0x1b;
    pub const ADDIW: u64 = 0x0;
    pub const SLLIW: u64 = 0x1;
    pub const SRIW: u64 = 0x5;
        pub const SRLIW: u64 = 0x00;
        pub const SRAIW: u64 = 0x20;

pub const R_TYPE_64: u64 = 0x3b;
    pub const ADDSUB: u64 = 0x0;
        pub const ADDW: u64 = 0x00;
        pub const MULW: u64 = 0x01;
        pub const SUBW: u64 = 0x20;
pub const DIVW: u64 = 0x4;
pub const SLLW: u64 = 0x1;
pub const SRW: u64 = 0x5;
    pub const SRLW: u64 = 0x00;
    pub const DIVUW: u64 = 0x01;
    pub const SRAW: u64 = 0x20;
pub const REMW:  u64 = 0x6;
pub const REMUW: u64 = 0x7;

pub const SYSTEM: u64 =       0x73;
pub const ECALLBREAK: u64 =   0x00;
pub const ECALL: u64 =            0x00;
pub const EBREAK: u64 =           0x01;
pub const MRET: u64 =             0x08;
pub const SRET: u64 =             0x18;
pub const CSRRW: u64 =        0x01;
pub const CSRRS: u64 =        0x02;
pub const CSRRC: u64 =        0x03;
pub const CSRRWI: u64 =       0x05;
pub const CSRRSI: u64 =       0x06;
pub const CSRRCI: u64 =       0x07;

pub const AMO_W:     u64 = 0x2f;
pub const LR_W:      u64 =    0x02;
pub const SC_W:      u64 =    0x03;
pub const AMOSWAP_W: u64 =    0x01;
pub const AMOADD_W:  u64 =    0x00;
pub const AMOXOR_W:  u64 =    0x04;
pub const AMOAND_W:  u64 =    0x0c;
pub const AMOOR_W:   u64 =    0x08;
pub const AMOMIN_W:  u64 =    0x10;
pub const AMOMAX_W:  u64 =    0x14;
pub const AMOMINU_W: u64 =    0x18;
pub const AMOMAXU_W: u64 =    0x1c;