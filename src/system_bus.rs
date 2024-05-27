use crate::dram::Dram;
use crate::errors::SystemBusError;

pub struct SystemBusMap {
    pub dram_base_addr: u64,
    pub dram_size: usize,
}

pub struct SystemBus {
    dram_base_addr: u64,
    dram_size: usize,
    dram: Dram,
}

impl SystemBus {
    pub fn new(map: SystemBusMap) -> Self {
        SystemBus {
            dram_base_addr: map.dram_base_addr,
            dram_size: map.dram_size,
            dram: Dram::new(map.dram_size),
        }
    }
}

impl SystemBus {
    pub fn bulk_store(&mut self, data: Vec<u8>) {
        self.dram.bulk_store(data);
    }

    pub fn bulk_store_segment(&mut self, data: Vec<u8>, addr: u64) {
        self.dram.bulk_store_segment(data, addr);
    }

    pub fn load(&self, addr: u64, size: usize) -> Result<u64, SystemBusError> {
        if addr >= self.dram_base_addr && addr < self.dram_base_addr + self.dram_size as u64 {
            Ok(self.dram.load(addr - self.dram_base_addr, size))
        } else {
            Err(SystemBusError::InvalidAddress)
        }
    }

    pub fn store(&mut self, data: u64, addr: u64, size: usize) -> Result<(), SystemBusError> {
        if addr >= self.dram_base_addr && addr < self.dram_base_addr + self.dram_size as u64 {
            self.dram.store(data, addr - self.dram_base_addr, size);
            Ok(())
        } else {
            Err(SystemBusError::InvalidAddress)
        }
    }
}
