use std::env;
use std::fs::File;
use std::io::{self, Read};

use librv64emu::Processor;
use librv64emu::system_bus::SystemBusMap;
use librv64emu::system_bus::SystemBus;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: j4frv32emu <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let sbus_map = SystemBusMap {
        dram_base_addr: 0x8000_0000,
        dram_size: 0x1_0000,
    };
    let mut sbus = SystemBus::new(sbus_map);
    sbus.bulk_store(data);

    let mut processor = Processor::new(sbus);
    while let Ok(()) = processor.tick() {
        continue;   
    }
    println!("{}", processor.dump());
    Ok(())
}