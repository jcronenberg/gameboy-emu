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

pub struct State8080 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: usize,
    pub pc: u16,
    pub memory: [u8; 0xffff],
    pub flags: Flags,
    pub int_enable: bool,
}

impl State8080 {
    pub fn new() -> State8080 {
        State8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: [0; 0xffff],
            flags: Flags::new(),
            int_enable: true,
        }
    }
}

macro_rules! N_TO_STR {
    ($a:expr) => {
        stringify!($a).chars()
                      .nth(6)
                      .unwrap()
    };
}

macro_rules! LD {
    ($a:expr,$b:expr) => {
        $a = $b;
        println!("LD {},{} {}: {:02x} {}: {:02x}", N_TO_STR!($a).to_uppercase(), N_TO_STR!($b).to_uppercase(),
                 N_TO_STR!($a), $a, N_TO_STR!($b), $b); //debug
    };
}

macro_rules! RL {
    ($a:expr,$b:expr) => {
        let tmp: u16 = if $b.flags.c { (($a as u16) << 1) + 1 } else { ($a as u16) << 1 };
        $a = (tmp & 0xff) as u8;
        $b.flags.z = $a == 0x0;
        $b.flags.n = false;
        $b.flags.h = false;
        $b.flags.c = 0x100 == tmp & 0x100;
        print!("RL {} {}: {:02x} ", N_TO_STR!($a).to_uppercase(), N_TO_STR!($a), $a); //debug
        print_flags!($b.flags); //debug
        println!(); //debug
    };
}

macro_rules! print_flags {
    ($a:expr) => {
        print!("flags: z: {}, n: {}, h: {}, c: {}", $a.z, $a.n, $a.h, $a.c);
    };
}

fn unimplemented_instruction(state: &State8080) {
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

/// INC register pair nn
/// Doesn't set any flags
fn inc_nn(reg1: &mut u8, reg2: &mut u8) {
    let mut reg12 = shift_nn(*reg1, *reg2);
    reg12 = reg12.wrapping_add(1);
    *reg1 = (reg12 >> 8) as u8;
    *reg2 = (reg12 & 0xff) as u8;
}

/// INC register n
fn inc_n(reg1: &mut u8, flags: &mut Flags) {
    flags.h = 0x10 == (*reg1 & 0xf).wrapping_add(1) & 0x10;
    *reg1 = reg1.wrapping_add(1);
    flags.z = *reg1 == 0;
    flags.n = false;
}

/// DEC register n
fn dec_n(reg1: &mut u8, flags: &mut Flags) {
    flags.h = 0x10 == (*reg1 & 0xf).wrapping_sub(1) & 0x10;
    *reg1 = reg1.wrapping_sub(1);
    flags.z = *reg1 == 0;
    flags.n = true;
}

/// DEC register pair nn
/// Doesn't set any flags
fn dec_nn(reg1: &mut u8, reg2: &mut u8) {
    let mut reg12 = shift_nn(*reg1, *reg2);
    reg12 = reg12.wrapping_sub(1);
    *reg1 = (reg12 >> 8) as u8;
    *reg2 = (reg12 & 0xff) as u8;
}

pub fn emulate_8080_op(state: &mut State8080) {
    // TODO fix for sm83
    // if state.pc >= 0x2000 {
    //     std::process::exit(0);
    // }
    let mut opcode: [u8; 3] = [0; 3];
    for i in 0..3 {
        opcode[i] = state.memory[state.pc as usize + i];
        // println!("{}: {}", i, opcode[i]); //debug
    }


    print!("{:04x} {:02x}: ", state.pc, opcode[0]); //debug

    state.pc += 1;

    match opcode[0] {
        0x00 => {}, //NOP
        0x01 => { //LD BC,d16
            state.c = opcode[1];
            state.b = opcode[2];
            state.pc += 2;
        },
        0x02 => { //LD (BC),A
            let bc = shift_nn(state.b, state.c);
            state.a = state.memory[bc as usize];
            println!("LD (BC),A bc: {:02x}, (bc): {:02x}, a: {:02x}", bc, state.memory[bc as usize], state.a) //debug
        },
        0x03 => { //INC BC
            inc_nn(&mut state.b, &mut state.c);
            println!("INC BC b: {:02x}, c: {:02x}", state.b, state.c); //debug
        },
        0x04 => { //INC B
            inc_n(&mut state.b, &mut state.flags);
            println!("INC B b: {:02x}, flags.z: {}, flags.h: {}", state.b, state.flags.z, state.flags.h); //debug
        },
        0x05 => { //DEC B
            dec_n(&mut state.b, &mut state.flags);
            println!("DEC B b: {:02x}, flags.z: {}, flags.h: {}", state.b, state.flags.z, state.flags.h); //debug
        },
        0x06 => { //LD B,d8
            state.b = opcode[1];
            state.pc += 1;
            println!("LD B,d8 b: {:02x}", state.b);
        },
        0x07 => {unimplemented_instruction(&state)},
        0x08 => {unimplemented_instruction(&state)},
        0x09 => {unimplemented_instruction(&state)},
        0x0a => {unimplemented_instruction(&state)},
        0x0b => {unimplemented_instruction(&state)},
        0x0c => { //INC C
            inc_n(&mut state.c, &mut state.flags);
            println!("INC C c: {:02x}, flags.z: {}, flags.h: {}", state.c, state.flags.z, state.flags.h); //debug
        },
        0x0d => {unimplemented_instruction(&state)},
        0x0e => { //LD C,d8
            state.c = opcode[1];
            state.pc += 1;
            println!("LD C,{:02x} c: {:02x}", opcode[1], state.c);
        },
        0x0f => {unimplemented_instruction(&state)},

        0x10 => { //STOP d8
            println!("Stopping not implemented, continuing...");
            state.pc += 2;
        },
        0x11 => { //LD DE,NN
            state.d = opcode[2];
            state.e = opcode[1];
            println!("LD DE d: {:02x}, e: {:02x}", state.d, state.e); //debug
            state.pc += 2;
        },
        0x12 => {unimplemented_instruction(&state)},
        0x13 => { //INC DE
            inc_nn(&mut state.d, &mut state.e);
            println!("INC DE d: {:02x}, e: {:02x}", state.d, state.e); //debug
        },
        0x14 => { //INC D
            inc_n(&mut state.d, &mut state.flags);
            println!("INC D d: {:02x}, flags.z: {}, flags.h: {}", state.d, state.flags.z, state.flags.h); //debug
        },
        0x15 => { //DEC D
            dec_n(&mut state.d, &mut state.flags);
            println!("DEC D d: {:02x}, flags.z: {}, flags.h: {}", state.d, state.flags.z, state.flags.h); //debug
        },
        0x16 => {unimplemented_instruction(&state)},
        0x17 => { //RLA
            RL!(state.a, state);
            state.flags.z = false;
            println!("(Above was RLA, flags.z = false)")
        },
        0x18 => {unimplemented_instruction(&state)},
        0x19 => {unimplemented_instruction(&state)},
        0x1a => { //LD A,(DE)
            state.a = state.memory[shift_nn(state.d, state.e) as usize];
            println!("LD A,(DE) a: {:02x}, ({:02x}): {:02x}", state.a, shift_nn(state.d, state.e),
                     state.memory[shift_nn(state.d, state.e) as usize]); //debug
        },
        0x1b => {unimplemented_instruction(&state)},
        0x1c => { //INC E
            inc_n(&mut state.e, &mut state.flags);
            println!("INC E e: {:02x}, flags.z: {}, flags.h: {}", state.e, state.flags.z, state.flags.h); //debug
        },
        0x1d => {unimplemented_instruction(&state)},
        0x1e => {unimplemented_instruction(&state)},
        0x1f => {unimplemented_instruction(&state)},

        0x20 => { //JR NZ,r8
            // TODO reevaluate
            // Dunno if this is the "best" way to do this
            // but it is necessary because normally this instruction is 2 bytes long
            state.pc += 1;
            if !state.flags.z {
                state.pc = state.pc.wrapping_add((opcode[1] as i8) as u16);
                println!("JR to {:04x}", state.pc) //debug
            } else {
                println!("No JR") //debug
            }
        },
        0x21 => { //LD HL,d16
            state.h = opcode[2];
            state.l = opcode[1];
            println!("LD HL h: {:02x}, l: {:02x}", state.h, state.l); //debug
            state.pc += 2;
        },
        0x22 => { //LD (HL+),A
            state.memory[shift_nn(state.h, state.l) as usize] = state.a;
            inc_nn(&mut state.h, &mut state.l);
            println!("LD (HL+),A h: {:02x}, l: {:02x}, a: {:02x}", state.h, state.l, state.a); //debug
        },
        0x23 => { //INC HL
            inc_nn(&mut state.h, &mut state.l);
            println!("INC HL h: {:02x}, l: {:02x}", state.h, state.l); //debug
        },
        0x24 => { // INC H
            inc_n(&mut state.h, &mut state.flags);
            println!("INC H h: {:02x}, flags.z: {}, flags.h: {}", state.h, state.flags.z, state.flags.h); //debug
        },
        0x25 => { //DEC H
            dec_n(&mut state.h, &mut state.flags);
            println!("DEC H h: {:02x}, flags.z: {}, flags.h: {}", state.h, state.flags.z, state.flags.h); //debug
        },
        0x26 => {unimplemented_instruction(&state)},
        0x27 => {unimplemented_instruction(&state)},
        0x28 => {unimplemented_instruction(&state)},
        0x29 => {unimplemented_instruction(&state)},
        0x2a => {unimplemented_instruction(&state)},
        0x2b => {unimplemented_instruction(&state)},
        0x2c => { //INC L
            inc_n(&mut state.l, &mut state.flags);
            println!("INC L l: {:02x}, flags.z: {}, flags.h: {}", state.l, state.flags.z, state.flags.h); //debug
        },
        0x2d => {unimplemented_instruction(&state)},
        0x2e => {unimplemented_instruction(&state)},
        0x2f => {unimplemented_instruction(&state)},

        0x30 => {unimplemented_instruction(&state)},
        0x31 => { //LD SP,NN
            state.sp = shift_nn(opcode[2], opcode[1]) as usize;
            println!("sp: {:04x}", state.sp); //debug
            state.pc += 2;
        },
        0x32 => { //LD (HL-),A
            state.memory[shift_nn(state.h, state.l) as usize] = state.a;
            dec_nn(&mut state.h, &mut state.l);
            println!("LD (HL-),A h: {:02x}, l: {:02x}, a: {:02x}", state.h, state.l, state.a); //debug
        },
        0x33 => { //INC SP
            state.sp = state.sp.wrapping_add(1);
        },
        0x34 => {unimplemented_instruction(&state)},
        0x35 => {unimplemented_instruction(&state)},
        0x36 => {unimplemented_instruction(&state)},
        0x37 => {unimplemented_instruction(&state)},
        0x38 => {unimplemented_instruction(&state)},
        0x39 => {unimplemented_instruction(&state)},
        0x3a => {unimplemented_instruction(&state)},
        0x3b => {unimplemented_instruction(&state)},
        0x3c => { //INC A
            inc_n(&mut state.a, &mut state.flags);
            println!("INC A a: {:02x}, flags.z: {}, flags.h: {}", state.a, state.flags.z, state.flags.h); //debug
        },
        0x3d => {unimplemented_instruction(&state)},
        0x3e => { //LD A,r8
            state.a = opcode[1];
            state.pc += 1;
            println!("LD A,{:02x} a: {:02x}", opcode[1], state.a); //debug
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
            unimplemented_instruction(&state);
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
            unimplemented_instruction(&state);
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
            unimplemented_instruction(&state);
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
            unimplemented_instruction(&state);
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
            unimplemented_instruction(&state);
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
            unimplemented_instruction(&state);
        },
        0x6f => { //LD L,A
            LD!(state.l, state.a);
        },

        0x70 => { //LD (HL),
            unimplemented_instruction(&state);
        },
        0x71 => { //LD (HL),
            unimplemented_instruction(&state);
        },
        0x72 => { //LD (HL),
            unimplemented_instruction(&state);
        },
        0x73 => { //LD (HL),
            unimplemented_instruction(&state);
        },
        0x74 => { //LD (HL),
            unimplemented_instruction(&state);
        },
        0x75 => { //LD (HL),
            unimplemented_instruction(&state);
        },
        0x76 => {unimplemented_instruction(&state)},
        0x77 => { //LD (HL),A
            state.memory[shift_nn(state.h, state.l) as usize] = state.a;
            println!("LD (HL),A ({:04x}): {:02x}, a: {:02x}", shift_nn(state.h, state.l),
                state.memory[shift_nn(state.h, state.l) as usize], state.a); //debug
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

        0x80 => {unimplemented_instruction(&state)},
        0x81 => {unimplemented_instruction(&state)},
        0x82 => {unimplemented_instruction(&state)},
        0x83 => {unimplemented_instruction(&state)},
        0x84 => {unimplemented_instruction(&state)},
        0x85 => {unimplemented_instruction(&state)},
        0x86 => {unimplemented_instruction(&state)},
        0x87 => {unimplemented_instruction(&state)},
        0x88 => {unimplemented_instruction(&state)},
        0x89 => {unimplemented_instruction(&state)},
        0x8a => {unimplemented_instruction(&state)},
        0x8b => {unimplemented_instruction(&state)},
        0x8c => {unimplemented_instruction(&state)},
        0x8d => {unimplemented_instruction(&state)},
        0x8e => {unimplemented_instruction(&state)},
        0x8f => {unimplemented_instruction(&state)},

        0x90 => {unimplemented_instruction(&state)},
        0x91 => {unimplemented_instruction(&state)},
        0x92 => {unimplemented_instruction(&state)},
        0x93 => {unimplemented_instruction(&state)},
        0x94 => {unimplemented_instruction(&state)},
        0x95 => {unimplemented_instruction(&state)},
        0x96 => {unimplemented_instruction(&state)},
        0x97 => {unimplemented_instruction(&state)},
        0x98 => {unimplemented_instruction(&state)},
        0x99 => {unimplemented_instruction(&state)},
        0x9a => {unimplemented_instruction(&state)},
        0x9b => {unimplemented_instruction(&state)},
        0x9c => {unimplemented_instruction(&state)},
        0x9d => {unimplemented_instruction(&state)},
        0x9e => {unimplemented_instruction(&state)},
        0x9f => {unimplemented_instruction(&state)},

        0xa0 => {unimplemented_instruction(&state)},
        0xa1 => {unimplemented_instruction(&state)},
        0xa2 => {unimplemented_instruction(&state)},
        0xa3 => {unimplemented_instruction(&state)},
        0xa4 => {unimplemented_instruction(&state)},
        0xa5 => {unimplemented_instruction(&state)},
        0xa6 => {unimplemented_instruction(&state)},
        0xa7 => {unimplemented_instruction(&state)},
        0xa8 => {unimplemented_instruction(&state)},
        0xa9 => {unimplemented_instruction(&state)},
        0xaa => {unimplemented_instruction(&state)},
        0xab => {unimplemented_instruction(&state)},
        0xac => {unimplemented_instruction(&state)},
        0xad => {unimplemented_instruction(&state)},
        0xae => {unimplemented_instruction(&state)},
        0xaf => { //XOR A
            state.a = 0x00;
            state.flags.z = true;
            state.flags.c = false;
            state.flags.h = false;
            state.flags.n = false;
        },

        0xb0 => {unimplemented_instruction(&state)},
        0xb1 => {unimplemented_instruction(&state)},
        0xb2 => {unimplemented_instruction(&state)},
        0xb3 => {unimplemented_instruction(&state)},
        0xb4 => {unimplemented_instruction(&state)},
        0xb5 => {unimplemented_instruction(&state)},
        0xb6 => {unimplemented_instruction(&state)},
        0xb7 => {unimplemented_instruction(&state)},
        0xb8 => {unimplemented_instruction(&state)},
        0xb9 => {unimplemented_instruction(&state)},
        0xba => {unimplemented_instruction(&state)},
        0xbb => {unimplemented_instruction(&state)},
        0xbc => {unimplemented_instruction(&state)},
        0xbd => {unimplemented_instruction(&state)},
        0xbe => {unimplemented_instruction(&state)},
        0xbf => {unimplemented_instruction(&state)},

        0xc0 => {unimplemented_instruction(&state)},
        0xc1 => { //POP BC
            state.c = state.memory[state.sp];
            state.b = state.memory[state.sp + 1];
            state.sp += 2;
            println!("POP BC b: {:02x}, c: {:02x}, sp: {:02x}", state.b, state.c, state.sp); //debug
        },
        0xc2 => { //JP NZ,a16
            if state.flags.z {
                state.pc = shift_nn(opcode[2], opcode[1]);
                println!("JP pc: {:04x}", state.pc); //debug
            } else {
                println!("JP skipped!"); //debug
            }
        },
        0xc3 => { //JP a16
            state.pc = shift_nn(opcode[2], opcode[1]);
            println!("JP pc: {:04x}", state.pc); //debug
        },
        0xc4 => {unimplemented_instruction(&state)},
        0xc5 => { //PUSH BC
            state.sp -= 2;
            state.memory[state.sp] = state.c;
            state.memory[state.sp + 1] = state.b;
            println!("PUSH BC (SP): {:02x}{:02x}, sp: {:02x}", state.memory[state.sp], state.memory[state.sp + 1], state.sp); //debug
        },
        0xc6 => {unimplemented_instruction(&state)},
        0xc7 => {unimplemented_instruction(&state)},
        0xc8 => {unimplemented_instruction(&state)},
        0xc9 => {unimplemented_instruction(&state)},
        0xca => {unimplemented_instruction(&state)},
        0xcb => { // PREFIX
            print!("Prefix: "); //debug
            state.pc += 1;
            match opcode[1] {
                // 0x01 => println!("0x01"),
                // 0x02 => println!("0x02"),
                // 0x03 => println!("0x03"),
                // 0x04 => println!("0x04"),
                // 0x05 => println!("0x05"),
                // 0x06 => println!("0x06"),
                // 0x07 => println!("0x07"),
                // 0x08 => println!("0x08"),
                // 0x09 => println!("0x09"),
                // 0x0a => println!("0x0a"),
                // 0x0b => println!("0x0b"),
                // 0x0c => println!("0x0c"),
                // 0x0d => println!("0x0d"),
                // 0x0e => println!("0x0e"),
                // 0x0f => println!("0x0f"),

                0x11 => { //RL C
                    RL!(state.c, state);
                },
                // 0x12 => println!("0x12"),
                // 0x13 => println!("0x13"),
                // 0x14 => println!("0x14"),
                // 0x15 => println!("0x15"),
                // 0x16 => println!("0x16"),
                // 0x17 => println!("0x17"),
                // 0x18 => println!("0x18"),
                // 0x19 => println!("0x19"),
                // 0x1a => println!("0x1a"),
                // 0x1b => println!("0x1b"),
                // 0x1c => println!("0x1c"),
                // 0x1d => println!("0x1d"),
                // 0x1e => println!("0x1e"),
                // 0x1f => println!("0x1f"),

                // 0x21 => println!("0x21"),
                // 0x22 => println!("0x22"),
                // 0x23 => println!("0x23"),
                // 0x24 => println!("0x24"),
                // 0x25 => println!("0x25"),
                // 0x26 => println!("0x26"),
                // 0x27 => println!("0x27"),
                // 0x28 => println!("0x28"),
                // 0x29 => println!("0x29"),
                // 0x2a => println!("0x2a"),
                // 0x2b => println!("0x2b"),
                // 0x2c => println!("0x2c"),
                // 0x2d => println!("0x2d"),
                // 0x2e => println!("0x2e"),
                // 0x2f => println!("0x2f"),

                // 0x31 => println!("0x31"),
                // 0x32 => println!("0x32"),
                // 0x33 => println!("0x33"),
                // 0x34 => println!("0x34"),
                // 0x35 => println!("0x35"),
                // 0x36 => println!("0x36"),
                // 0x37 => println!("0x37"),
                // 0x38 => println!("0x38"),
                // 0x39 => println!("0x39"),
                // 0x3a => println!("0x3a"),
                // 0x3b => println!("0x3b"),
                // 0x3c => println!("0x3c"),
                // 0x3d => println!("0x3d"),
                // 0x3e => println!("0x3e"),
                // 0x3f => println!("0x3f"),

                // 0x41 => println!("0x41"),
                // 0x42 => println!("0x42"),
                // 0x43 => println!("0x43"),
                // 0x44 => println!("0x44"),
                // 0x45 => println!("0x45"),
                // 0x46 => println!("0x46"),
                // 0x47 => println!("0x47"),
                // 0x48 => println!("0x48"),
                // 0x49 => println!("0x49"),
                // 0x4a => println!("0x4a"),
                // 0x4b => println!("0x4b"),
                // 0x4c => println!("0x4c"),
                // 0x4d => println!("0x4d"),
                // 0x4e => println!("0x4e"),
                // 0x4f => println!("0x4f"),

                // 0x51 => println!("0x51"),
                // 0x52 => println!("0x52"),
                // 0x53 => println!("0x53"),
                // 0x54 => println!("0x54"),
                // 0x55 => println!("0x55"),
                // 0x56 => println!("0x56"),
                // 0x57 => println!("0x57"),
                // 0x58 => println!("0x58"),
                // 0x59 => println!("0x59"),
                // 0x5a => println!("0x5a"),
                // 0x5b => println!("0x5b"),
                // 0x5c => println!("0x5c"),
                // 0x5d => println!("0x5d"),
                // 0x5e => println!("0x5e"),
                // 0x5f => println!("0x5f"),

                // 0x61 => println!("0x61"),
                // 0x62 => println!("0x62"),
                // 0x63 => println!("0x63"),
                // 0x64 => println!("0x64"),
                // 0x65 => println!("0x65"),
                // 0x66 => println!("0x66"),
                // 0x67 => println!("0x67"),
                // 0x68 => println!("0x68"),
                // 0x69 => println!("0x69"),
                // 0x6a => println!("0x6a"),
                // 0x6b => println!("0x6b"),
                // 0x6c => println!("0x6c"),
                // 0x6d => println!("0x6d"),
                // 0x6e => println!("0x6e"),
                // 0x6f => println!("0x6f"),

                // 0x71 => println!("0x71"),
                // 0x72 => println!("0x72"),
                // 0x73 => println!("0x73"),
                // 0x74 => println!("0x74"),
                // 0x75 => println!("0x75"),
                // 0x76 => println!("0x76"),
                // 0x77 => println!("0x77"),
                // 0x78 => println!("0x78"),
                // 0x79 => println!("0x79"),
                // 0x7a => println!("0x7a"),
                // 0x7b => println!("0x7b"),
                0x7c => { // BIT 7,H
                    state.flags.z = 0b01000000 == state.h & 0b01000000;
                    state.flags.n = false;
                    state.flags.h = true;
                    println!("BIT 7,H flags.z: {}", state.flags.z); //debug
                },
                // 0x7d => println!("0x7d"),
                // 0x7e => println!("0x7e"),
                // 0x7f => println!("0x7f"),

                // 0x81 => println!("0x81"),
                // 0x82 => println!("0x82"),
                // 0x83 => println!("0x83"),
                // 0x84 => println!("0x84"),
                // 0x85 => println!("0x85"),
                // 0x86 => println!("0x86"),
                // 0x87 => println!("0x87"),
                // 0x88 => println!("0x88"),
                // 0x89 => println!("0x89"),
                // 0x8a => println!("0x8a"),
                // 0x8b => println!("0x8b"),
                // 0x8c => println!("0x8c"),
                // 0x8d => println!("0x8d"),
                // 0x8e => println!("0x8e"),
                // 0x8f => println!("0x8f"),

                // 0x91 => println!("0x91"),
                // 0x92 => println!("0x92"),
                // 0x93 => println!("0x93"),
                // 0x94 => println!("0x94"),
                // 0x95 => println!("0x95"),
                // 0x96 => println!("0x96"),
                // 0x97 => println!("0x97"),
                // 0x98 => println!("0x98"),
                // 0x99 => println!("0x99"),
                // 0x9a => println!("0x9a"),
                // 0x9b => println!("0x9b"),
                // 0x9c => println!("0x9c"),
                // 0x9d => println!("0x9d"),
                // 0x9e => println!("0x9e"),
                // 0x9f => println!("0x9f"),

                // 0xa1 => println!("0xa1"),
                // 0xa2 => println!("0xa2"),
                // 0xa3 => println!("0xa3"),
                // 0xa4 => println!("0xa4"),
                // 0xa5 => println!("0xa5"),
                // 0xa6 => println!("0xa6"),
                // 0xa7 => println!("0xa7"),
                // 0xa8 => println!("0xa8"),
                // 0xa9 => println!("0xa9"),
                // 0xaa => println!("0xaa"),
                // 0xab => println!("0xab"),
                // 0xac => println!("0xac"),
                // 0xad => println!("0xad"),
                // 0xae => println!("0xae"),
                // 0xaf => println!("0xaf"),

                // 0xb1 => println!("0xb1"),
                // 0xb2 => println!("0xb2"),
                // 0xb3 => println!("0xb3"),
                // 0xb4 => println!("0xb4"),
                // 0xb5 => println!("0xb5"),
                // 0xb6 => println!("0xb6"),
                // 0xb7 => println!("0xb7"),
                // 0xb8 => println!("0xb8"),
                // 0xb9 => println!("0xb9"),
                // 0xba => println!("0xba"),
                // 0xbb => println!("0xbb"),
                // 0xbc => println!("0xbc"),
                // 0xbd => println!("0xbd"),
                // 0xbe => println!("0xbe"),
                // 0xbf => println!("0xbf"),

                // 0xc1 => println!("0xc1"),
                // 0xc2 => println!("0xc2"),
                // 0xc3 => println!("0xc3"),
                // 0xc4 => println!("0xc4"),
                // 0xc5 => println!("0xc5"),
                // 0xc6 => println!("0xc6"),
                // 0xc7 => println!("0xc7"),
                // 0xc8 => println!("0xc8"),
                // 0xc9 => println!("0xc9"),
                // 0xca => println!("0xca"),
                // 0xcb => println!("0xcb"),
                // 0xcc => println!("0xcc"),
                // 0xcd => println!("0xcd"),
                // 0xce => println!("0xce"),
                // 0xcf => println!("0xcf"),

                // 0xd1 => println!("0xd1"),
                // 0xd2 => println!("0xd2"),
                // 0xd3 => println!("0xd3"),
                // 0xd4 => println!("0xd4"),
                // 0xd5 => println!("0xd5"),
                // 0xd6 => println!("0xd6"),
                // 0xd7 => println!("0xd7"),
                // 0xd8 => println!("0xd8"),
                // 0xd9 => println!("0xd9"),
                // 0xda => println!("0xda"),
                // 0xdb => println!("0xdb"),
                // 0xdc => println!("0xdc"),
                // 0xdd => println!("0xdd"),
                // 0xde => println!("0xde"),
                // 0xdf => println!("0xdf"),

                // 0xe1 => println!("0xe1"),
                // 0xe2 => println!("0xe2"),
                // 0xe3 => println!("0xe3"),
                // 0xe4 => println!("0xe4"),
                // 0xe5 => println!("0xe5"),
                // 0xe6 => println!("0xe6"),
                // 0xe7 => println!("0xe7"),
                // 0xe8 => println!("0xe8"),
                // 0xe9 => println!("0xe9"),
                // 0xea => println!("0xea"),
                // 0xeb => println!("0xeb"),
                // 0xec => println!("0xec"),
                // 0xed => println!("0xed"),
                // 0xee => println!("0xee"),
                // 0xef => println!("0xef"),

                // 0xf1 => println!("0xf1"),
                // 0xf2 => println!("0xf2"),
                // 0xf3 => println!("0xf3"),
                // 0xf4 => println!("0xf4"),
                // 0xf5 => println!("0xf5"),
                // 0xf6 => println!("0xf6"),
                // 0xf7 => println!("0xf7"),
                // 0xf8 => println!("0xf8"),
                // 0xf9 => println!("0xf9"),
                // 0xfa => println!("0xfa"),
                // 0xfb => println!("0xfb"),
                // 0xfc => println!("0xfc"),
                // 0xfd => println!("0xfd"),
                // 0xfe => println!("0xfe"),
                // 0xff => println!("0xff"),
                _ => unimplemented_instruction(&state),
            }
        },
        0xcc => {unimplemented_instruction(&state)},
        0xcd => { //CALL NN
            // TODO this may be incorrect, check this maybe later
            state.sp -= 2;
            state.memory[state.sp] = opcode[2];
            state.memory[state.sp + 1] = opcode[1];
            state.pc = shift_nn(opcode[2], opcode[1]);
            println!("CALL NN nn: {:02x}{:02x}, pc: {:02x}, sp: {:02x} (sp): {:02x}{:02x}",
                     opcode[2], opcode[1], state.pc, state.sp, state.memory[state.sp], state.memory[state.sp + 1]); //debug
        },
        0xce => {unimplemented_instruction(&state)},
        0xcf => {unimplemented_instruction(&state)},

        0xd0 => {unimplemented_instruction(&state)},
        0xd1 => { //POP DE
            state.e = state.memory[state.sp];
            state.d = state.memory[state.sp + 1];
            state.sp += 2;
            println!("POP DE d: {:02x}, e: {:02x}, sp: {:02x}", state.d, state.e, state.sp); //debug
        },
        0xd2 => {unimplemented_instruction(&state)},
        0xd3 => {unimplemented_instruction(&state)},
        0xd4 => {unimplemented_instruction(&state)},
        0xd5 => { //PUSH DE
            state.sp -= 2;
            state.memory[state.sp] = state.e;
            state.memory[state.sp + 1] = state.d;
            println!("PUSH DE (SP): {:02x}{:02x}, sp: {:02x}", state.memory[state.sp], state.memory[state.sp + 1], state.sp); //debug
        },
        0xd6 => {unimplemented_instruction(&state)},
        0xd7 => {unimplemented_instruction(&state)},
        0xd8 => {unimplemented_instruction(&state)},
        0xd9 => {unimplemented_instruction(&state)},
        0xda => {unimplemented_instruction(&state)},
        0xdb => {unimplemented_instruction(&state)},
        0xdc => {unimplemented_instruction(&state)},
        0xdd => {unimplemented_instruction(&state)},
        0xde => {unimplemented_instruction(&state)},
        0xdf => {unimplemented_instruction(&state)},

        0xe0 => { //LDH (a8),A
            state.memory[(0xff00 & opcode[1] as u16) as usize] = state.a;
            println!("LD (0xff00+a8),A (0xff{:02x}): {:02x}, a: {:02x}",
                     opcode[1], state.memory[(0xff00 & opcode[1] as u16) as usize], state.a); //debug
            state.pc += 1;
        },
        0xe1 => { //POP HL
            state.l = state.memory[state.sp];
            state.h = state.memory[state.sp + 1];
            state.sp += 2;
            println!("POP HL h: {:02x}, l: {:02x}, sp: {:02x}", state.h, state.l, state.sp); //debug
        },
        0xe2 => { //LD (C),A
            state.memory[(0xff00 & state.c as u16) as usize] = state.a;
            println!("LD (0xff00+C),A (0xff{:02x}): {:02x}, a: {:02x}",
                     state.c, state.memory[(0xff00 & state.c as u16) as usize], state.a); //debug
        },
        0xe3 => {unimplemented_instruction(&state)},
        0xe4 => {unimplemented_instruction(&state)},
        0xe5 => { //PUSH HL
            state.sp -= 2;
            state.memory[state.sp] = state.l;
            state.memory[state.sp + 1] = state.h;
            println!("PUSH HL (SP): {:02x}{:02x}, sp: {:02x}", state.memory[state.sp], state.memory[state.sp + 1], state.sp); //debug
        },
        0xe6 => {unimplemented_instruction(&state)},
        0xe7 => {unimplemented_instruction(&state)},
        0xe8 => {unimplemented_instruction(&state)},
        0xe9 => {unimplemented_instruction(&state)},
        0xea => {unimplemented_instruction(&state)},
        0xeb => {unimplemented_instruction(&state)},
        0xec => {unimplemented_instruction(&state)},
        0xed => {unimplemented_instruction(&state)},
        0xee => {unimplemented_instruction(&state)},
        0xef => {unimplemented_instruction(&state)},

        0xf0 => {unimplemented_instruction(&state)},
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
        0xfc => {unimplemented_instruction(&state)},
        0xfd => {unimplemented_instruction(&state)},
        0xfe => {unimplemented_instruction(&state)},
        0xff => {unimplemented_instruction(&state)},
        // _ => unreachable!()
    }
}
