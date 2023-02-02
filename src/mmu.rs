use std::io;
use std::io::prelude::*;
use std::fs::File;

use crate::sm83cpu;

pub struct MMU {
    rom_type: u8,
    boot_finished: bool,
    boot_rom: [u8; 0x100],
    cart: [u8; 0x8000],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            rom_type: 0,
            boot_finished: false,
            boot_rom: [0; 0x100],
            cart: [0; 0x8000],
        }
    }

    pub fn load_boot_rom(&mut self) {
        let mut buffer = Vec::new();
        match read_file_to_buf("boot.bin", &mut buffer) {
            Ok(()) => println!("Successfully loaded boot rom"),
            Err(i) => {
                panic!("Error loading boot rom: {}", i);
            }
        }
        for i in 0..buffer.len() {
            self.boot_rom[i] = buffer[i];
        }
    }

    pub fn load_cart(&mut self, cart_name: &str) {
        let mut buffer = Vec::new();
        match read_file_to_buf(cart_name, &mut buffer) {
            Ok(()) => println!("Successfully loaded cartridge {}", cart_name),
            Err(i) => {
                panic!("Error loading cartridge {}: {}", cart_name, i);
            }
        }
        for i in 0..buffer.len() {
            self.cart[i] = buffer[i];
        }
    }

    pub fn overwrite_boot_rom(&mut self, state: &mut sm83cpu::StateSM83) {
        if self.boot_finished { return }
        for i in 0..self.boot_rom.len() {
            state.memory[i] = self.cart[i];
        }
        self.boot_finished = true;
        #[cfg(debug_assertions)] println!("Overwritten boot rom");
    }

    pub fn cart_to_mem(&self, state: &mut sm83cpu::StateSM83) {
        for i in 0..self.cart.len() {
            state.memory[i] = self.cart[i];
        }
    }

    pub fn boot_rom_to_mem(&self, state: &mut sm83cpu::StateSM83) {
        for i in 0..self.boot_rom.len() {
            state.memory[i] = self.boot_rom[i];
        }
    }

    pub fn read_header(&mut self) {
        self.rom_type = self.cart[0x147];
        #[cfg(debug_assertions)] println!("Set rom_type to {:02x}", self.rom_type);
    }
}

fn read_file_to_buf(file: &str, buffer: &mut Vec<u8>) -> io::Result<()> {
    let mut f = File::open(file)?;

    f.read_to_end(buffer)?;

    Ok(())
}
