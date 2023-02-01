use std::io;
use std::env;
use std::io::prelude::*;
use std::fs::File;

mod disassembler;
mod sm83cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        usage();
        return
    } else if args.len() != 3 {
        usage();
        std::process::exit(1);
    }

    let mut buffer = Vec::new();
    match read_file_to_buf(&args[2], &mut buffer) {
        Ok(()) => println!("Successfully loaded file: {}", args[2]),
        Err(i) => {
            panic!("Error loading file '{}': {}", args[2], i);
        }
    }

    if args[1] == "hexdump" {
        disassembler::hexdump(buffer);
    } else if args[1] == "disassemble" {
        let length = buffer.len();
        let mut i:usize = 0;
        while i < length {
            i += disassembler::disassemble_SM83_op(&buffer, i);
        }
    } else if args[1] == "emulate" {
        let mut state = sm83cpu::StateSM83::new();

        //Load memory
        for i in 0..buffer.len() {
            state.memory[i] = buffer[i];
        }

        //Main Loop
        loop {
            sm83cpu::emulate_sm83_op(&mut state);
        }
    } else {
        println!("Unknown command!\n");
        usage();
        std::process::exit(1);
    }
}

fn read_file_to_buf(file: &str, buffer: &mut Vec<u8>) -> io::Result<()> {
    let mut f = File::open(file)?;

    f.read_to_end(buffer)?;

    Ok(())
}

fn usage() {
    println!("USAGE: gameboy-emu <command> <file>\n");
    println!("COMMANDS:");
    println!("disassemble   disassemble file and output to stdout");
    println!("hexdump       hexdump file and output to stdout");
    println!("emulate       emulate file")
}
