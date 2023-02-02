use crate::mmu;

pub struct Flags {
    pub z: bool,
    // pub s: bool,
    // pub p: u8,
    pub c: bool,
    pub n: bool,
    pub h: bool,
    // pub ac: u8,
    // pub pad: u8,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            z: true,
            // s: true,
            // p: 1,
            c: true,
            n: true,
            h: true,
            // ac: 1,
            // pad: 3,
        }
    }
}

pub struct StateSM83 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: usize,
    pub pc: u16,
    pub memory: [u8; 0x10000],
    pub flags: Flags,
    pub int_enable: bool,
}

impl StateSM83 {
    pub fn new() -> StateSM83 {
        StateSM83 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: [0; 0x10000],
            flags: Flags::new(),
            int_enable: true,
        }
    }
}

macro_rules! N_TO_STR {
    ($n:expr) => {
        stringify!($n).chars()
                      .nth(6)
                      .unwrap()
    };
}

// TODO debug statement can be nonsense
macro_rules! LD {
    ($address1:expr,$address2:expr) => {
        $address1 = $address2;
        #[cfg(debug_assertions)] println!("LD {},{} {}: {:02x} {}: {:02x}", N_TO_STR!($address1).to_uppercase(), N_TO_STR!($address2).to_uppercase(),
                 N_TO_STR!($address1), $address1, N_TO_STR!($address2), $address2);
    };
}

macro_rules! RL {
    ($address:expr,$state:expr) => {
        let tmp: u16 = if $state.flags.c { (($address as u16) << 1) + 1 } else { ($address as u16) << 1 };
        $address = (tmp & 0xff) as u8;
        $state.flags.z = $address == 0x0;
        $state.flags.n = false;
        $state.flags.h = false;
        $state.flags.c = 0x100 == tmp & 0x100;
        #[cfg(debug_assertions)] print!("RL {} {}: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! INC {
    ($address:expr,$state:expr) => {
        $state.flags.h = 0x10 == ($address & 0xf).wrapping_add(1) & 0x10;
        $address = $address.wrapping_add(1);
        $state.flags.z = $address == 0;
        $state.flags.n = false;
        #[cfg(debug_assertions)] print!("INC {} {}: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
    // $c is unused but is necessary to differentiate
    ($address1:expr,$address1ddress2:expr,$state:expr) => {
        let mut cmb = shift_nn($address1, $address1ddress2);
        cmb = cmb.wrapping_add(1);
        $address1 = (cmb >> 8) as u8;
        $address1ddress2 = (cmb & 0xff) as u8;
        #[cfg(debug_assertions)] println!("INC {}{} {}: {:02x}, {}: {:02x}", N_TO_STR!($address1).to_uppercase(), N_TO_STR!($address1ddress2).to_uppercase(),
                 N_TO_STR!($address1), $address1, N_TO_STR!($address1ddress2), $address1ddress2);
    };
}

macro_rules! DEC {
    ($address:expr,$state:expr) => {
        $state.flags.h = 0x10 == ($address & 0xf).wrapping_sub(1) & 0x10;
        $address = $address.wrapping_sub(1);
        $state.flags.z = $address == 0;
        $state.flags.n = true;
        #[cfg(debug_assertions)] print!("DEC {} {}: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
    // $state is unused but is necessary to differentiate
    ($address1:expr,$address2:expr,$state:expr) => {
        let mut cmb = shift_nn($address1, $address2);
        cmb = cmb.wrapping_sub(1);
        $address1 = (cmb >> 8) as u8;
        $address2 = (cmb & 0xff) as u8;
        #[cfg(debug_assertions)] println!("DEC {}{} {}: {:02x}, {}: {:02x}", N_TO_STR!($address1).to_uppercase(), N_TO_STR!($address2).to_uppercase(),
                 N_TO_STR!($address1), $address1, N_TO_STR!($address2), $address2);
    };
}

macro_rules! SUB {
    ($address:expr,$state:expr) => {
        $state.flags.h = 0x10 == ($state.a & 0xf).wrapping_sub($address & 0xf) & 0x10;
        let tmp: u16 = ($state.a as u16).wrapping_sub($address as u16);
        $state.flags.c = 0x100 == tmp & 0x100;
        $state.flags.n = true;
        $state.a = tmp as u8;
        $state.flags.z = $state.a == 0x0;
        #[cfg(debug_assertions)] print!("SUB {} {}: {:02x} a: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address, $state.a);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! SBC {
    ($address:expr,$state:expr) => {
        $state.flags.h = 0x10 == ($state.a & 0xf).wrapping_sub($address.wrapping_sub($state.flags.c as u8) & 0xf) & 0x10;
        let tmp: u16 = ($state.a as u16).wrapping_sub($address as u16);
        $state.flags.c = 0x100 == tmp & 0x100;
        $state.flags.n = true;
        $state.a = tmp as u8;
        $state.flags.z = $state.a == 0x0;
        #[cfg(debug_assertions)] print!("SBC {} {}: {:02x} a: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address, $state.a);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! ADD {
    ($address:expr,$state:expr) => {
        $state.flags.h = 0x10 == ($state.a & 0xf).wrapping_add($address & 0xf) & 0x10;
        let tmp: u16 = ($state.a as u16).wrapping_add($address as u16);
        $state.flags.c = 0x100 == tmp & 0x100;
        $state.flags.n = true;
        $state.a = tmp as u8;
        $state.flags.z = $state.a == 0x0;
        #[cfg(debug_assertions)] print!("ADD {} {}: {:02x} a: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address, $state.a);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! ADC {
    ($address:expr,$state:expr) => {
        $state.flags.h = 0x10 == ($state.a & 0xf).wrapping_add($address.wrapping_add($state.flags.c as u8) & 0xf) & 0x10;
        let tmp: u16 = ($state.a as u16).wrapping_add($address as u16);
        $state.flags.c = 0x100 == tmp & 0x100;
        $state.flags.n = true;
        $state.a = tmp as u8;
        $state.flags.z = $state.a == 0x0;
        #[cfg(debug_assertions)] print!("ADC {} {}: {:02x} a: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address, $state.a);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! AND {
    ($address:expr,$state:expr) => {
        $state.a &= $address;
        $state.flags.z = 0x0 == $state.a;
        $state.flags.c = false;
        $state.flags.h = true;
        $state.flags.n = false;
        #[cfg(debug_assertions)] print!("AND {} {}: {:02x} a: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address, $state.a);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! XOR {
    ($address:expr,$state:expr) => {
        $state.a ^= $address;
        $state.flags.z = 0x0 == $state.a;
        $state.flags.c = false;
        $state.flags.h = false;
        $state.flags.n = false;
        #[cfg(debug_assertions)] print!("XOR {} {}: {:02x} a: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address, $state.a);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! OR {
    ($address:expr,$state:expr) => {
        $state.a |= $address;
        $state.flags.z = 0x0 == $state.a;
        $state.flags.c = false;
        $state.flags.h = false;
        $state.flags.n = false;
        #[cfg(debug_assertions)] print!("OR {} {}: {:02x} a: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address, $state.a);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! CP {
    ($address:expr,$state:expr) => {
        let tmp = $state.a;
        let tmp2: u16 = (tmp as u16).wrapping_sub($address as u16);
        $state.flags.c = (tmp2 & 0xff00) > 0x0;
        $state.flags.h = 0x10 == (tmp & 0xf).wrapping_sub($address & 0xf) & 0x10;
        $state.flags.z = tmp2 == 0x0;
        $state.flags.n = true;
        #[cfg(debug_assertions)] print!("CP {} {}: {:02x} a: {:02x} ", N_TO_STR!($address).to_uppercase(), N_TO_STR!($address), $address, $state.a);
        #[cfg(debug_assertions)] print_flags!($state.flags);
        #[cfg(debug_assertions)] println!();
    };
}

macro_rules! M {
    ($address1:expr,$address2:expr,$state:expr) => {
        $state.memory[shift_nn($address1, $address2) as usize]
    };
    ($address1:expr,$state:expr) => {
        $state.memory[(0xff00 + $address1 as u16) as usize]
    };
}

macro_rules! print_flags {
    ($flags:expr) => {
        print!("flags: z: {}, n: {}, h: {}, c: {}", $flags.z, $flags.n, $flags.h, $flags.c);
    };
}

fn unimplemented_instruction(state: &StateSM83) {
    println!("\nInstruction: 0x{:02x}/{}", state.memory[state.pc as usize - 1],
             state.memory[state.pc as usize - 1]);
    println!("PC: {:04x}", state.pc - 1);
    println!("Not implemented yet! Exiting...");
    std::process::exit(1);
}

/// Takes two 8 bit numbers and shifts the first to be the first
/// 8 bits of a 16 bit number and the second as the last 8 bits
fn shift_nn(shift1: u8, shift2: u8) -> u16 {
    let mut tmp: u16;
    tmp = (shift1 as u16) << 8;
    tmp |= shift2 as u16;
    return tmp;
}

pub fn emulate_sm83_op(state: &mut StateSM83, mmu: &mut mmu::MMU) {
    // TODO fix for sm83
    // if state.pc >= 0x2000 {
    //     std::process::exit(0);
    // }
    let mut opcode: [u8; 3] = [0; 3];
    for i in 0..3 {
        opcode[i] = state.memory[state.pc as usize + i];
        // println!("{}: {}", i, opcode[i]);
    }


    #[cfg(debug_assertions)] print!("{:04x} {:02x}: ", state.pc, opcode[0]);

    if state.pc == 0x100 { mmu.overwrite_boot_rom(state) }

    state.pc += 1;

    match opcode[0] {
        0x00 => {}, //NOP
        0x01 => { //LD BC,d16
            state.c = opcode[1];
            state.b = opcode[2];
            state.pc += 2;
        },
        0x02 => { //LD (BC),A
            LD!(M!(state.b, state.c, state), state.a);
        },
        0x03 => { //INC BC
            INC!(state.b, state.c, state);
        },
        0x04 => { //INC B
            INC!(state.b, state);
        },
        0x05 => { //DEC B
            DEC!(state.b, state);
        },
        0x06 => { //LD B,d8
            state.pc += 1;
            LD!(state.b, opcode[1]);
        },
        0x07 => {unimplemented_instruction(&state)},
        0x08 => {unimplemented_instruction(&state)},
        0x09 => {unimplemented_instruction(&state)},
        0x0a => {unimplemented_instruction(&state)},
        0x0b => { //DEC BC
            DEC!(state.b, state.c, state);
        },
        0x0c => { //INC C
            INC!(state.c, state);
        },
        0x0d => { //DEC C
            DEC!(state.c, state);
        },
        0x0e => { //LD C,d8
            state.pc += 1;
            LD!(state.c, opcode[1]);
        },
        0x0f => {unimplemented_instruction(&state)},

        0x10 => { //STOP d8
            println!("Stopping not implemented, continuing...");
            state.pc += 1;
        },
        0x11 => { //LD DE,NN
            state.d = opcode[2];
            state.e = opcode[1];
            #[cfg(debug_assertions)] println!("LD DE d: {:02x}, e: {:02x}", state.d, state.e);
            state.pc += 2;
        },
        0x12 => {unimplemented_instruction(&state)},
        0x13 => { //INC DE
            INC!(state.d, state.e, state);
        },
        0x14 => { //INC D
            INC!(state.d, state);
        },
        0x15 => { //DEC D
            DEC!(state.d, state);
        },
        0x16 => { //LD D,d8
            state.pc += 1;
            LD!(state.d, opcode[1]);
        },
        0x17 => { //RLA
            RL!(state.a, state);
            state.flags.z = false;
            #[cfg(debug_assertions)] println!("(Above was RLA, flags.z = false)");
        },
        0x18 => { //JR r8
            state.pc += 1;
            state.pc = state.pc.wrapping_add((opcode[1] as i8) as u16);
            #[cfg(debug_assertions)] println!("JR to {:04x}", state.pc);
        },
        0x19 => {unimplemented_instruction(&state)},
        0x1a => { //LD A,(DE)
            LD!(state.a, M!(state.d, state.e, state));
        },
        0x1b => { //DEC DE
            DEC!(state.d, state.e, state);
        },
        0x1c => { //INC E
            INC!(state.e, state);
        },
        0x1d => { //DEC E
            DEC!(state.e, state);
        },
        0x1e => { //LD E,d8
            state.pc += 1;
            LD!(state.e, opcode[1]);
        },
        0x1f => {unimplemented_instruction(&state)},

        0x20 => { //JR NZ,d8
            state.pc += 1;
            if !state.flags.z {
                state.pc = state.pc.wrapping_add((opcode[1] as i8) as u16);
                #[cfg(debug_assertions)] println!("JR to {:04x}", state.pc)
            } else {
                #[cfg(debug_assertions)] println!("No JR")
            }
        },
        0x21 => { //LD HL,d16
            state.h = opcode[2];
            state.l = opcode[1];
            #[cfg(debug_assertions)] println!("LD HL h: {:02x}, l: {:02x}", state.h, state.l);
            state.pc += 2;
        },
        0x22 => { //LD (HL+),A
            LD!(M!(state.h, state.l, state), state.a);
            INC!(state.h, state.l, state);
        },
        0x23 => { //INC HL
            INC!(state.h, state.l, state);
        },
        0x24 => { // INC H
            INC!(state.h, state);
        },
        0x25 => { //DEC H
            DEC!(state.h, state);
        },
        0x26 => { //LD H,d8
            state.pc += 1;
            LD!(state.h, opcode[1]);
        },
        0x27 => {unimplemented_instruction(&state)},
        0x28 => { //JR Z,r8
            state.pc += 1;
            if state.flags.z {
                state.pc = state.pc.wrapping_add((opcode[1] as i8) as u16);
                #[cfg(debug_assertions)] println!("JR to {:04x}", state.pc)
            } else {
                #[cfg(debug_assertions)] println!("No JR")
            }
        },
        0x29 => {unimplemented_instruction(&state)},
        0x2a => { // LD A,(HL+)
            LD!(state.a, M!(state.h, state.l, state));
            INC!(state.h, state.l, state);
        },
        0x2b => { //DEC HL
            DEC!(state.h, state.l, state);
        },
        0x2c => { //INC L
            INC!(state.l, state);
        },
        0x2d => { //DEC L
            DEC!(state.l, state);
        },
        0x2e => { //LD L,d8
            state.pc += 1;
            LD!(state.l, opcode[1]);
        },
        0x2f => {unimplemented_instruction(&state)},

        0x30 => {unimplemented_instruction(&state)},
        0x31 => { //LD SP,NN
            state.sp = shift_nn(opcode[2], opcode[1]) as usize;
            #[cfg(debug_assertions)] println!("sp: {:04x}", state.sp);
            state.pc += 2;
        },
        0x32 => { //LD (HL-),A
            LD!(M!(state.h, state.l, state), state.a);
            DEC!(state.h, state.l, state);
        },
        0x33 => { //INC SP
            state.sp = state.sp.wrapping_add(1);
        },
        0x34 => { //INC (HL)
            INC!(M!(state.h, state.l, state), state);
        },
        0x35 => { //DEC (HL)
            DEC!(M!(state.h, state.l, state), state);
            println!("Above was DEC (HL)");
        },
        0x36 => { //LD (HL),d8
            state.pc += 1;
            LD!(M!(state.h, state.l, state), opcode[1]);
        },
        0x37 => {unimplemented_instruction(&state)},
        0x38 => {unimplemented_instruction(&state)},
        0x39 => {unimplemented_instruction(&state)},
        0x3a => { //LD A,(HL-)
            LD!(state.a, M!(state.h, state.l, state));
            DEC!(state.h, state.l, state);
        },
        0x3b => { //DEC SP
            // TODO reevaluate
            // This may not work because sp is usize, so burrows will probably not work correctly
            DEC!(state.sp, state);
        },
        0x3c => { //INC A
            INC!(state.a, state);
        },
        0x3d => { //DEC A
            DEC!(state.a, state);
        },
        0x3e => { //LD A,d8
            state.pc += 1;
            LD!(state.a, opcode[1]);
        },
        0x3f => {unimplemented_instruction(&state)},

        0x40 => { //LD B,B
            LD!(state.b, state.b);
        },
        0x41 => { //LD B,C
            LD!(state.b, state.c);
        },
        0x42 => { //LD B,D
            LD!(state.b, state.d);
        },
        0x43 => { //LD B,E
            LD!(state.b, state.e);
        },
        0x44 => { //LD B,H
            LD!(state.b, state.h);
        },
        0x45 => { //LD B,L
            LD!(state.b, state.l);
        },
        0x46 => { //LD B,(HL)
            LD!(state.b, M!(state.h, state.l, state));
        },
        0x47 => { //LD B,A
            LD!(state.b, state.a);
        },
        0x48 => { //LD C,B
            LD!(state.c, state.b);
        },
        0x49 => { //LD C,C
            LD!(state.c, state.c);
        },
        0x4a => { //LD C,D
            LD!(state.c, state.d);
        },
        0x4b => { //LD C,E
            LD!(state.c, state.e);
        },
        0x4c => { //LD C,H
            LD!(state.c, state.h);
        },
        0x4d => { //LD C,L
            LD!(state.c, state.l);
        },
        0x4e => { //LD C,(HL)
            LD!(state.c, M!(state.h, state.l, state));
        },
        0x4f => { //LD C,A
            LD!(state.c, state.a);
        },

        0x50 => { //LD D,B
            LD!(state.d, state.b);
        },
        0x51 => { //LD D,C
            LD!(state.d, state.c);
        },
        0x52 => { //LD D,D
            LD!(state.d, state.d);
        },
        0x53 => { //LD D,E
            LD!(state.d, state.e);
        },
        0x54 => { //LD D,H
            LD!(state.d, state.h);
        },
        0x55 => { //LD D,L
            LD!(state.d, state.l);
        },
        0x56 => { //LD D,(HL)
            LD!(state.d, M!(state.h, state.l, state));
        },
        0x57 => { //LD D,A
            LD!(state.d, state.a);
        },
        0x58 => { //LD E,B
            LD!(state.e, state.b);
        },
        0x59 => { //LD E,C
            LD!(state.e, state.c);
        },
        0x5a => { //LD E,D
            LD!(state.e, state.d);
        },
        0x5b => { //LD E,E
            LD!(state.e, state.e);
        },
        0x5c => { //LD E,H
            LD!(state.e, state.h);
        },
        0x5d => { //LD E,L
            LD!(state.e, state.l);
        },
        0x5e => { //LD E,(HL)
            LD!(state.e, M!(state.h, state.l, state));
        },
        0x5f => { //LD E,A
            LD!(state.e, state.a);
        },

        0x60 => { //LD H,B
            LD!(state.h, state.b);
        },
        0x61 => { //LD H,C
            LD!(state.h, state.c);
        },
        0x62 => { //LD H,D
            LD!(state.h, state.d);
        },
        0x63 => { //LD H,E
            LD!(state.h, state.e);
        },
        0x64 => { //LD H,H
            LD!(state.h, state.h);
        },
        0x65 => { //LD H,L
            LD!(state.h, state.l);
        },
        0x66 => { //LD H,(HL)
            LD!(state.h, M!(state.h, state.l, state));
        },
        0x67 => { //LD H,A
            LD!(state.h, state.a);
        },
        0x68 => { //LD L,B
            LD!(state.l, state.b);
        },
        0x69 => { //LD L,C
            LD!(state.l, state.c);
        },
        0x6a => { //LD L,D
            LD!(state.l, state.d);
        },
        0x6b => { //LD L,E
            LD!(state.l, state.e);
        },
        0x6c => { //LD L,H
            LD!(state.l, state.h);
        },
        0x6d => { //LD L,L
            LD!(state.l, state.l);
        },
        0x6e => { //LD L,(HL)
            LD!(state.l, M!(state.h, state.l, state));
        },
        0x6f => { //LD L,A
            LD!(state.l, state.a);
        },

        0x70 => { //LD (HL),B
            LD!(M!(state.h, state.l, state), state.b);
        },
        0x71 => { //LD (HL),C
            LD!(M!(state.h, state.l, state), state.c);
        },
        0x72 => { //LD (HL),D
            LD!(M!(state.h, state.l, state), state.d);
        },
        0x73 => { //LD (HL),E
            LD!(M!(state.h, state.l, state), state.e);
        },
        0x74 => { //LD (HL),H
            LD!(M!(state.h, state.l, state), state.h);
        },
        0x75 => { //LD (HL),L
            LD!(M!(state.h, state.l, state), state.l);
        },
        0x76 => { //HALT
            println!("HALT not implemented");
        },
        0x77 => { //LD (HL),A
            LD!(M!(state.h, state.l, state), state.a);
        },
        0x78 => { //LD A,B
            LD!(state.a, state.b);
        },
        0x79 => { //LD A,C
            LD!(state.a, state.c);
        },
        0x7a => { //LD A,D
            LD!(state.a, state.d);
        },
        0x7b => { //LD A,E
            LD!(state.a, state.e);
        },
        0x7c => { //LD A,H
            LD!(state.a, state.h);
        },
        0x7d => { //LD A,L
            LD!(state.a, state.l);
        },
        0x7e => { //LD A,(HL)
            unimplemented_instruction(&state);
        },
        0x7f => { //LD A,A
            LD!(state.a, state.a);
        },

        0x80 => { //ADD A,B
            ADD!(state.b, state);
        },
        0x81 => { //ADD A,C
            ADD!(state.c, state);
        },
        0x82 => { //ADD A,D
            ADD!(state.d, state);
        },
        0x83 => { //ADD A,E
            ADD!(state.e, state);
        },
        0x84 => { //ADD A,H
            ADD!(state.h, state);
        },
        0x85 => { //ADD A,L
            ADD!(state.l, state);
        },
        0x86 => { //ADD A,(HL)
            ADD!(M!(state.h, state.l, state), state);
        },
        0x87 => { //ADD A,A
            ADD!(state.a, state);
        },
        0x88 => { //ADC A,B
            ADC!(state.b, state);
        },
        0x89 => { //ADC A,C
            ADC!(state.c, state);
        },
        0x8a => { //ADC A,D
            ADC!(state.d, state);
        },
        0x8b => { //ADC A,E
            ADC!(state.e, state);
        },
        0x8c => { //ADC A,H
            ADC!(state.h, state);
        },
        0x8d => { //ADC A,L
            ADC!(state.l, state);
        },
        0x8e => { //ADC A,(HL)
            ADC!(M!(state.h, state.l, state), state);
        },
        0x8f => { //ADC A,A
            ADC!(state.b, state);
        },

        0x90 => { //SUB B
            SUB!(state.b, state);
        },
        0x91 => { //SUB C
            SUB!(state.c, state);
        },
        0x92 => { //SUB D
            SUB!(state.d, state);
        },
        0x93 => { //SUB E
            SUB!(state.e, state);
        },
        0x94 => { //SUB H
            SUB!(state.h, state);
        },
        0x95 => { //SUB L
            SUB!(state.l, state);
        },
        0x96 => { //SUB (HL)
            SUB!(M!(state.h, state.l, state), state);
        },
        0x97 => { //SUB A
            SUB!(state.a, state);
        },
        0x98 => { //SBC A,B
            SBC!(state.b, state);
        },
        0x99 => { //SBC A,C
            SBC!(state.c, state);
        },
        0x9a => { //SBC A,D
            SBC!(state.d, state);
        },
        0x9b => { //SBC A,E
            SBC!(state.e, state);
        },
        0x9c => { //SBC A,H
            SBC!(state.h, state);
        },
        0x9d => { //SBC A,L
            SBC!(state.l, state);
        },
        0x9e => { //SBC A,(HL)
            SBC!(M!(state.h, state.l, state), state);
        },
        0x9f => { //SBC A,A
            SBC!(state.a, state);
        },

        0xa0 => { //AND B
            AND!(state.b, state);
        },
        0xa1 => { //AND C
            AND!(state.c, state);
        },
        0xa2 => { //AND D
            AND!(state.d, state);
        },
        0xa3 => { //AND E
            AND!(state.e, state);
        },
        0xa4 => { //AND H
            AND!(state.h, state);
        },
        0xa5 => { //AND L
            AND!(state.l, state);
        },
        0xa6 => { //AND (HL)
            AND!(M!(state.h, state.l, state), state);
        },
        0xa7 => { //AND A
            AND!(state.a, state);
        },
        0xa8 => { //XOR B
            XOR!(state.b, state);
        },
        0xa9 => { //XOR C
            XOR!(state.c, state);
        },
        0xaa => { //XOR D
            XOR!(state.d, state);
        },
        0xab => { //XOR E
            XOR!(state.e, state);
        },
        0xac => { //XOR H
            XOR!(state.h, state);
        },
        0xad => { //XOR L
            XOR!(state.l, state);
        },
        0xae => { //XOR (HL)
            XOR!(M!(state.h, state.l, state), state);
        },
        0xaf => { //XOR A
            XOR!(state.a, state);
        },

        0xb0 => { //OR B
            OR!(state.b, state);
        },
        0xb1 => { //OR C
            OR!(state.c, state);
        },
        0xb2 => { //OR D
            OR!(state.d, state);
        },
        0xb3 => { //OR E
            OR!(state.e, state);
        },
        0xb4 => { //OR H
            OR!(state.h, state);
        },
        0xb5 => { //OR L
            OR!(state.l, state);
        },
        0xb6 => { //OR (HL)
            OR!(M!(state.h, state.l, state), state);
        },
        0xb7 => { //OR A
            OR!(state.a, state);
        },
        0xb8 => { //CP B
            CP!(state.b, state);
        },
        0xb9 => { //CP C
            CP!(state.c, state);
        },
        0xba => { //CP D
            CP!(state.d, state);
        },
        0xbb => { //CP E
            CP!(state.e, state);
        },
        0xbc => { //CP H
            CP!(state.h, state);
        },
        0xbd => { //CP L
            CP!(state.l, state);
        },
        0xbe => { //CP (HL)
            CP!(M!(state.h, state.l, state), state);
        },
        0xbf => { //CP A
            CP!(state.a, state);
        },

        0xc0 => { //RET NZ
            if !state.flags.z {
                state.pc = state.memory[state.sp] as u16;
                state.pc |= (state.memory[state.sp + 1] as u16) << 8;
                #[cfg(debug_assertions)] println!("RET NZ memory: {:04x}", (state.memory[state.sp] as u16) | (state.memory[state.sp + 1] as u16) << 8);
                state.sp += 2;
                #[cfg(debug_assertions)] println!("RET NZ pc: {:04x}", state.pc);
            } else { #[cfg(debug_assertions)] println!("RET NZ not returned"); }
        },
        0xc1 => { //POP BC
            state.c = state.memory[state.sp];
            state.b = state.memory[state.sp + 1];
            state.sp += 2;
            #[cfg(debug_assertions)] println!("POP BC b: {:02x}, c: {:02x}, sp: {:02x}", state.b, state.c, state.sp);
        },
        0xc2 => { //JP NZ,a16
            if state.flags.z {
                state.pc = shift_nn(opcode[2], opcode[1]);
                #[cfg(debug_assertions)] println!("JP pc: {:04x}", state.pc);
            } else { #[cfg(debug_assertions)] println!("JP skipped!"); }
        },
        0xc3 => { //JP a16
            state.pc = shift_nn(opcode[2], opcode[1]);
            #[cfg(debug_assertions)] println!("JP pc: {:04x}", state.pc);
        },
        0xc4 => { //CALL NZ,a16
            if !state.flags.z {
                // TODO this may be incorrect, check this maybe later
                state.pc += 2;
                state.sp -= 2;
                state.memory[state.sp] = (state.pc & 0xff) as u8;
                state.memory[state.sp + 1] = ((state.pc & 0xff00) >> 8) as u8;
                state.pc = shift_nn(opcode[2], opcode[1]);
                #[cfg(debug_assertions)] println!("CALL NZ,NN nn: {:02x}{:02x}, pc: {:04x}, sp: {:02x} (sp): {:02x}{:02x}",
                                                  opcode[2], opcode[1], state.pc, state.sp, state.memory[state.sp + 1], state.memory[state.sp]);
            } else { #[cfg(debug_assertions)] println!("CALL NZ skipped"); }
        },
        0xc5 => { //PUSH BC
            state.sp -= 2;
            state.memory[state.sp] = state.c;
            state.memory[state.sp + 1] = state.b;
            #[cfg(debug_assertions)] println!("PUSH BC (SP): {:02x}{:02x}, sp: {:02x}", state.memory[state.sp], state.memory[state.sp + 1], state.sp);
        },
        0xc6 => {unimplemented_instruction(&state)},
        0xc7 => {unimplemented_instruction(&state)},
        0xc8 => { //RET Z
            if state.flags.z {
                state.pc = state.memory[state.sp] as u16;
                state.pc |= (state.memory[state.sp + 1] as u16) << 8;
                #[cfg(debug_assertions)] println!("RET Z memory: {:04x}", (state.memory[state.sp] as u16) | (state.memory[state.sp + 1] as u16) << 8);
                state.sp += 2;
                #[cfg(debug_assertions)] println!("RET Z pc: {:04x}", state.pc);
            } else { #[cfg(debug_assertions)] println!("RET Z not returned"); }
        },
        0xc9 => { //RET
            state.pc = state.memory[state.sp] as u16;
            state.pc |= (state.memory[state.sp + 1] as u16) << 8;
            #[cfg(debug_assertions)] println!("RET memory: {:04x}", (state.memory[state.sp] as u16) | (state.memory[state.sp + 1] as u16) << 8);
            state.sp += 2;
            #[cfg(debug_assertions)] println!("RET pc: {:04x}", state.pc);
        },
        0xca => {unimplemented_instruction(&state)},
        0xcb => { // PREFIX
            #[cfg(debug_assertions)] print!("Prefix: ");
            state.pc += 1;
            let register = opcode[1] & 0x7;
            let mut val: u8 = match register {
                0 => state.b,
                1 => state.c,
                2 => state.d,
                3 => state.e,
                4 => state.h,
                5 => state.l,
                6 => M!(state.h, state.l, state),
                7 => state.a,
                _ => unreachable!()
            };

            let mut write: bool = true;
            match opcode[1] >> 4 {
                0x0 => unimplemented_instruction(state),

                0x1 => {
                    if opcode[1] >> 3 & 0x1 == 0 {
                        let tmp: u16 = if state.flags.c { ((val as u16) << 1) | 0x1 } else { (val as u16) << 1 };
                        val = (tmp & 0xff) as u8;
                        state.flags.z = val == 0x0;
                        state.flags.n = false;
                        state.flags.h = false;
                        state.flags.c = 0x100 == tmp & 0x100;
                        #[cfg(debug_assertions)] print!("RL {:02x} {:02x}: {:02x} ", register, register, val);
                        #[cfg(debug_assertions)] print_flags!(state.flags);
                        #[cfg(debug_assertions)] println!();
                    } else {
                        unimplemented_instruction(state);
                    }
                },
                0x4 |
                0x5 |
                0x6 |
                0x7 => {
                    // This gets the position of the bit to be checked
                    // e.g. for 0x78-0x7f it would be 7
                    // Then we can bitshift 1 by that amount to check this bit
                    let bit: u8 = opcode[1] >> 3 & 0x7;
                    state.flags.z = 0 == (val & (1 << bit));
                    state.flags.n = false;
                    state.flags.h = true;
                    write = false;
                    #[cfg(debug_assertions)] println!("BIT {},{:02x} flags.z: {}", bit, register, state.flags.z);
                },
                _ => unimplemented_instruction(&state),
            }
            if write {
                match register {
                    0 => state.b = val,
                    1 => state.c = val,
                    2 => state.d = val,
                    3 => state.e = val,
                    4 => state.h = val,
                    5 => state.l = val,
                    6 => M!(state.h, state.l, state) = val,
                    7 => state.a = val,
                    _ => unreachable!()
                };
            }
        },
        0xcc => { //CALL Z,a16
            if state.flags.z {
                // TODO this may be incorrect, check this maybe later
                state.pc += 2;
                state.sp -= 2;
                state.memory[state.sp] = (state.pc & 0xff) as u8;
                state.memory[state.sp + 1] = ((state.pc & 0xff00) >> 8) as u8;
                state.pc = shift_nn(opcode[2], opcode[1]);
                #[cfg(debug_assertions)] println!("CALL Z,NN nn: {:02x}{:02x}, pc: {:04x}, sp: {:02x} (sp): {:02x}{:02x}",
                                                  opcode[2], opcode[1], state.pc, state.sp, state.memory[state.sp + 1], state.memory[state.sp]);
            } else { #[cfg(debug_assertions)] println!("CALL Z skipped"); }
        },
        0xcd => { //CALL NN
            // TODO this may be incorrect, check this maybe later
            state.pc += 2;
            state.sp -= 2;
            state.memory[state.sp] = (state.pc & 0xff) as u8;
            state.memory[state.sp + 1] = ((state.pc & 0xff00) >> 8) as u8;
            state.pc = shift_nn(opcode[2], opcode[1]);
            #[cfg(debug_assertions)] println!("CALL NN nn: {:02x}{:02x}, pc: {:04x}, sp: {:02x} (sp): {:02x}{:02x}",
                     opcode[2], opcode[1], state.pc, state.sp, state.memory[state.sp + 1], state.memory[state.sp]);
        },
        0xce => {unimplemented_instruction(&state)},
        0xcf => {unimplemented_instruction(&state)},

        0xd0 => { //RET NC
            if !state.flags.c {
                state.pc = state.memory[state.sp] as u16;
                state.pc |= (state.memory[state.sp + 1] as u16) << 8;
                #[cfg(debug_assertions)] println!("RET NC memory: {:04x}", (state.memory[state.sp] as u16) | (state.memory[state.sp + 1] as u16) << 8);
                state.sp += 2;
                #[cfg(debug_assertions)] println!("RET NC pc: {:04x}", state.pc);
            } else { #[cfg(debug_assertions)] println!("RET NC not returned"); }
        },
        0xd1 => { //POP DE
            state.e = state.memory[state.sp];
            state.d = state.memory[state.sp + 1];
            state.sp += 2;
            #[cfg(debug_assertions)] println!("POP DE d: {:02x}, e: {:02x}, sp: {:02x}", state.d, state.e, state.sp);
        },
        0xd2 => {unimplemented_instruction(&state)},
        0xd3 => {unimplemented_instruction(&state)},
        0xd4 => { //CALL NC,a16
            if !state.flags.c {
                // TODO this may be incorrect, check this maybe later
                state.pc += 2;
                state.sp -= 2;
                state.memory[state.sp] = (state.pc & 0xff) as u8;
                state.memory[state.sp + 1] = ((state.pc & 0xff00) >> 8) as u8;
                state.pc = shift_nn(opcode[2], opcode[1]);
                #[cfg(debug_assertions)] println!("CALL NC,NN nn: {:02x}{:02x}, pc: {:04x}, sp: {:02x} (sp): {:02x}{:02x}",
                                                  opcode[2], opcode[1], state.pc, state.sp, state.memory[state.sp + 1], state.memory[state.sp]);
            } else { #[cfg(debug_assertions)] println!("CALL NC skipped"); }
        },
        0xd5 => { //PUSH DE
            state.sp -= 2;
            state.memory[state.sp] = state.e;
            state.memory[state.sp + 1] = state.d;
            #[cfg(debug_assertions)] println!("PUSH DE (SP): {:02x}{:02x}, sp: {:02x}", state.memory[state.sp], state.memory[state.sp + 1], state.sp);
        },
        0xd6 => {unimplemented_instruction(&state)},
        0xd7 => {unimplemented_instruction(&state)},
        0xd8 => { //RET C
            if state.flags.c {
                state.pc = state.memory[state.sp] as u16;
                state.pc |= (state.memory[state.sp + 1] as u16) << 8;
                #[cfg(debug_assertions)] println!("RET NC memory: {:04x}", (state.memory[state.sp] as u16) | (state.memory[state.sp + 1] as u16) << 8);
                state.sp += 2;
                #[cfg(debug_assertions)] println!("RET NC pc: {:04x}", state.pc);
            } else { #[cfg(debug_assertions)] println!("RET NC not returned"); }
        },
        0xd9 => {unimplemented_instruction(&state)},
        0xda => {unimplemented_instruction(&state)},
        0xdb => { //no instruction
            unimplemented_instruction(&state)
        },
        0xdc => { //CALL C,a16
            if state.flags.c {
                // TODO this may be incorrect, check this maybe later
                state.pc += 2;
                state.sp -= 2;
                state.memory[state.sp] = (state.pc & 0xff) as u8;
                state.memory[state.sp + 1] = ((state.pc & 0xff00) >> 8) as u8;
                state.pc = shift_nn(opcode[2], opcode[1]);
                #[cfg(debug_assertions)] println!("CALL C,NN nn: {:02x}{:02x}, pc: {:04x}, sp: {:02x} (sp): {:02x}{:02x}",
                                                  opcode[2], opcode[1], state.pc, state.sp, state.memory[state.sp + 1], state.memory[state.sp]);
            } else { #[cfg(debug_assertions)] println!("CALL C skipped"); }
        },
        0xdd => { //no instruction
            unimplemented_instruction(&state)
        },
        0xde => {unimplemented_instruction(&state)},
        0xdf => {unimplemented_instruction(&state)},

        0xe0 => { //LDH (a8),A [LD (0xff00+a8),A]
            state.pc += 1;
            LD!(M!(opcode[1], state), state.a);
            #[cfg(debug_assertions)] println!("(above was)LD (0xff00+a8),A ({:04x}): {:02x} a: {:02x}", 0xff00 + opcode[1] as u16,
                     state.memory[(0xff00 + opcode[1] as u16) as usize], state.a);
        },
        0xe1 => { //POP HL
            state.l = state.memory[state.sp];
            state.h = state.memory[state.sp + 1];
            state.sp += 2;
            #[cfg(debug_assertions)] println!("POP HL h: {:02x}, l: {:02x}, sp: {:02x}", state.h, state.l, state.sp);
        },
        0xe2 => { //LD (C),A
            state.memory[(0xff00 & state.c as u16) as usize] = state.a;
            #[cfg(debug_assertions)] println!("LD (0xff00+C),A (0xff{:02x}): {:02x}, a: {:02x}",
                     state.c, state.memory[(0xff00 & state.c as u16) as usize], state.a);
        },
        0xe3 => { //no instruction
            unimplemented_instruction(&state)
        },
        0xe4 => { //no instruction
            unimplemented_instruction(&state)
        },
        0xe5 => { //PUSH HL
            state.sp -= 2;
            state.memory[state.sp] = state.l;
            state.memory[state.sp + 1] = state.h;
            #[cfg(debug_assertions)] println!("PUSH HL (SP): {:02x}{:02x}, sp: {:02x}", state.memory[state.sp], state.memory[state.sp + 1], state.sp);
        },
        0xe6 => {unimplemented_instruction(&state)},
        0xe7 => {unimplemented_instruction(&state)},
        0xe8 => {unimplemented_instruction(&state)},
        0xe9 => {unimplemented_instruction(&state)},
        0xea => { // LD (a16),A
            state.pc += 2;
            LD!(state.memory[shift_nn(opcode[2], opcode[1]) as usize], state.a);
            #[cfg(debug_assertions)] println!("(above was)LD (a16),A ({:04x}): {:02x} a: {:02x}", shift_nn(opcode[2], opcode[1]),
                     state.memory[shift_nn(opcode[2], opcode[1]) as usize], state.a);
        },
        0xeb => { //no instruction
            unimplemented_instruction(&state)
        },
        0xec => { //no instruction
            unimplemented_instruction(&state)
        },
        0xed => { //no instruction
            unimplemented_instruction(&state)
        },
        0xee => {unimplemented_instruction(&state)},
        0xef => {unimplemented_instruction(&state)},

        0xf0 => { //LDH A,(a8) [LD A,(0xff00+a8)]
            state.pc += 1;
            LD!(state.a, state.memory[(0xff00 + opcode[1] as u16) as usize]);
            #[cfg(debug_assertions)] println!("(above was)LD A,(0xff00+a8) a: {:02x} ({:04x}): {:02x}", state.a,
                     0xff00 + opcode[1] as u16, state.memory[(0xff00 + opcode[1] as u16) as usize]);

            //TODO tmp
            // Since the boot sequence waits for the screen but this isn't implemented
            // We just do it manually
            if 0xff00 + opcode[1] as u16 == 0xff44 {
                state.a = 0x90;
                #[cfg(debug_assertions)] println!("tmp set a to 0x90");
            }
        },
        0xf1 => {unimplemented_instruction(&state)},
        0xf2 => {unimplemented_instruction(&state)},
        0xf3 => {unimplemented_instruction(&state)},
        0xf4 => {unimplemented_instruction(&state)},
        0xf5 => {unimplemented_instruction(&state)},
        0xf6 => {unimplemented_instruction(&state)},
        0xf7 => {unimplemented_instruction(&state)},
        0xf8 => {unimplemented_instruction(&state)},
        0xf9 => {unimplemented_instruction(&state)},
        0xfa => {unimplemented_instruction(&state)},
        0xfb => {unimplemented_instruction(&state)},
        0xfc => { //no instruction
            unimplemented_instruction(&state);
        },
        0xfd => { //no instruction
            unimplemented_instruction(&state);
        },
        0xfe => { //CP d8
            state.pc += 1;
            CP!(opcode[1], state);
        },
        0xff => {unimplemented_instruction(&state)},
        // _ => unreachable!()
    }
}
