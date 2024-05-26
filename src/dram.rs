#![allow(dead_code)]
#![allow(unused_variables)]

pub struct Dram {
    mem: Vec<u8>,
}

impl Dram {
    pub fn new(size: usize) -> Self {
        Dram {
            mem: vec![0u8; size],
        }
    }

    pub fn bulk_store(&mut self, data: Vec<u8>) {
        self.mem = data;
    }

    pub fn bulk_store_segment(&mut self, data: Vec<u8>, addr: u64) {
        let addr = addr;
        let mut tmp = Vec::from(&self.mem[..addr as usize]);
        tmp.extend(data.as_slice());
        tmp.extend(&self.mem[addr as usize + data.len()..]);
        self.mem = tmp;
    }

    pub fn load_8(&self, addr: u64) -> u64 {
        return self.mem[addr as usize] as u64;
    }

    pub fn load_16(&self, addr: u64) -> u64 {
        return self.mem[addr as usize] as u64
            |  ((self.mem[addr as usize + 1] as u64) << 8);
    }

    pub fn load_32(&self, addr: u64) -> u64 {
        return self.mem[addr as usize] as u64
            |  ((self.mem[addr as usize + 1] as u64) << 8)
            |  ((self.mem[addr as usize + 2] as u64) << 16)
            |  ((self.mem[addr as usize + 3] as u64) << 24);
    }

    pub fn load_64(&self, addr: u64) -> u64 {
        return self.mem[addr as usize] as u64
            |  ((self.mem[addr as usize + 1] as u64) << 8)
            |  ((self.mem[addr as usize + 2] as u64) << 16)
            |  ((self.mem[addr as usize + 3] as u64) << 24)
            |  ((self.mem[addr as usize + 4] as u64) << 32)
            |  ((self.mem[addr as usize + 5] as u64) << 40)
            |  ((self.mem[addr as usize + 6] as u64) << 48)
            |  ((self.mem[addr as usize + 7] as u64) << 56);
    }

    pub fn load(&self, addr: u64, size: usize) -> u64 {
        match size {
            8  => self.load_8(addr),
            16 => self.load_16(addr),
            32 => self.load_32(addr),
            64 => self.load_64(addr),
            _ => todo!(),
        }
    }

    pub fn store_8(&mut self, data: u64, addr: u64) {
        self.mem[addr as usize] = (data & 0xff) as u8;
    }

    pub fn store_16(&mut self, data: u64, addr: u64) {
        self.mem[addr as usize] = (data & 0xff) as u8;
        self.mem[(addr + 1) as usize] = ((data >> 8) & 0xff) as u8;
    }

    pub fn store_32(&mut self, data: u64, addr: u64) {
        self.mem[addr as usize] = (data & 0xff) as u8;
        self.mem[(addr + 1) as usize] = ((data >> 8) & 0xff) as u8;
        self.mem[(addr + 2) as usize] = ((data >> 16) & 0xff) as u8;
        self.mem[(addr + 3) as usize] = ((data >> 24) & 0xff) as u8;
    }

    pub fn store_64(&mut self, data: u64, addr: u64) {
        self.mem[(addr + 0) as usize] = ((data >>  0) & 0xff) as u8;
        self.mem[(addr + 1) as usize] = ((data >>  8) & 0xff) as u8;
        self.mem[(addr + 2) as usize] = ((data >> 16) & 0xff) as u8;
        self.mem[(addr + 3) as usize] = ((data >> 24) & 0xff) as u8;
        self.mem[(addr + 4) as usize] = ((data >> 32) & 0xff) as u8;
        self.mem[(addr + 5) as usize] = ((data >> 40) & 0xff) as u8;
        self.mem[(addr + 6) as usize] = ((data >> 48) & 0xff) as u8;
        self.mem[(addr + 7) as usize] = ((data >> 56) & 0xff) as u8;
    }

    pub fn store(&mut self, data: u64, addr: u64, size: usize) {
        match size {
            1 => self.store_8(data, addr),
            2 => self.store_16(data, addr),
            4 => self.store_32(data, addr),
            8 => self.store_64(data, addr),
            _ => todo!(),
        }
    }
}


