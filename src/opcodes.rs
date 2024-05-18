
const LUI: usize = 0x37; 
const AUIPC: usize = 0x17; 

const JAL: usize = 0x6f;
const JALR: usize = 0x67;

const B_TYPE: usize = 0x63;
const BEQ: usize = 0x0;
const BNE: usize = 0x1;
const BLT: usize = 0x4;
const BGE: usize = 0x5;
const BLTU: usize = 0x6;
const BGEU: usize = 0x7;

const LOAD: usize = 0x03;
const LB: usize = 0x0;
const LH: usize = 0x1;
const LW: usize = 0x2;
const LD: usize = 0x3;
const LBU: usize = 0x4;
const LHU: usize = 0x5;
const LWU: usize = 0x6;

const S_TYPE: usize = 0x23;
const SB: usize = 0x0;
const SH: usize = 0x1;
const SW: usize = 0x2;
const SD: usize = 0x3;

const R_TYPE_64: usize = 0x3b;
    const ADDSUB: usize = 0x0;
        const ADDW: usize = 0x00;
        const MULW: usize = 0x01;
        const SUBW: usize = 0x20;
const DIVW: usize = 0x4;
const SLLW: usize = 0x1;
const SRW: usize = 0x5;
    const SRLW: usize = 0x00;
    const DIVUW: usize = 0x01;
    const SRAW: usize = 0x20;
const REMW:  usize = 0x6;
const REMUW: usize = 0x7;

const CSR: usize =      0x73;
const ECALLBREAK: usize =   0x00;
const CSRRW: usize =        0x01;
const CSRRS: usize =        0x02;
const CSRRC: usize =        0x03;
const CSRRWI: usize =       0x05;
const CSRRSI: usize =       0x06;
const CSRRCI: usize =       0x07;

const AMO_W:     usize = 0x2f;
const LR_W:      usize =    0x02;
const SC_W:      usize =    0x03;
const AMOSWAP_W: usize =    0x01;
const AMOADD_W:  usize =    0x00;
const AMOXOR_W:  usize =    0x04;
const AMOAND_W:  usize =    0x0c;
const AMOOR_W:   usize =    0x08;
const AMOMIN_W:  usize =    0x10;
const AMOMAX_W:  usize =    0x14;
const AMOMINU_W: usize =    0x18;
const AMOMAXU_W: usize =    0x1c;