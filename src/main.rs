use std::env;
use std::fs::File;
use std::io::{self, Read};

use processor::Processor;

mod processor;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: j4frv32emu <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut processor = Processor::new();
    processor.load(data);
    while !processor.is_halted() {
        processor.tick();
    }
    println!("{}", processor.dump());
    Ok(())
}
