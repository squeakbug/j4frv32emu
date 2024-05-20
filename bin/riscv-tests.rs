use std::io;
use std::path::PathBuf;

use colored::Colorize;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use glob::glob;

use librv64emu::errors::ProcessorError;
use librv64emu::Processor;

fn run_test(path: &PathBuf) -> io::Result<()> {
    print!("test: {}: ", path.to_str().expect("Bad path"));

    let file_data = std::fs::read(path)?;
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Bad parse");
    let segments =  file.segments().expect("Bad program headers");

    let mut processor = Processor::new();
    for segment in segments.iter().skip(1) {
        let data = file.segment_data(&segment).expect("Bad parse segment data");
        processor.load_to(data.to_vec(), segment.p_paddr);
    }
    processor.set_pc(0x8000_0000);

    while let Ok(()) = processor.tick() {
        continue;   
    }
    match processor.tick() {
        Err(ProcessorError::BufferOverflow) => println!("{}", "OK".green()),
        _ => println!("{}", "ERROR".red()),
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
