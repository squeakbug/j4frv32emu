use std::io;
use std::path::PathBuf;

use colored::Colorize;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use glob::glob;

use librv64emu::errors::*;
use librv64emu::Processor;
use librv64emu::system_bus::SystemBusMap;
use librv64emu::system_bus::SystemBus;

fn run_test(path: &PathBuf) -> io::Result<()> {
    print!("test: {}: ", path.to_str().expect("Bad path"));

    let file_data = std::fs::read(path)?;
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Bad parse");
    let segments =  file.segments().expect("Bad program headers");

    let sbus_map = SystemBusMap {
        dram_base_addr: 0x8000_0000,
        dram_size: 0x1_0000,
    };
    let mut sbus = SystemBus::new(sbus_map);

    for segment in segments.iter().skip(1) {
        let data = file.segment_data(&segment).expect("Bad parse segment data");
        sbus.bulk_store_segment(data.to_vec(), segment.p_paddr - 0x8000_0000);
    }
    let mut processor = Processor::new(sbus);
    processor.set_pc(0x8000_0000);

    while let Ok(()) = processor.tick() {
        continue;   
    }
    match processor.tick() {
        Err(ProcessorError::BufferOverflow) => println!("{}", "OK".green()),
        _ => println!("{}\n{}", "ERROR".red(), processor.dump()),
    };
    Ok(())
}

fn main() -> io::Result<()> {
    for entry in glob("riscv-tests/isa/rv64ui-p-addi").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.to_str().unwrap().ends_with(".dump") {
                    continue;
                }
                let _ = run_test(&path);
            },
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}
