use crate::sm83cpu;

pub fn disassemble_sm83_op(buffer: &Vec<u8>, pc: usize) -> usize {
    let mut opbytes:usize = 1;

    //Print current pc
    print!("{:04x},{:02x}: ", pc, buffer[pc]);

    match buffer[pc] {
        0x00 => println!("NOP"),
        0x01 => {println!("LD     B,#${:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            opbytes = 3},
        0x02 => println!("STAX   B"),
        0x03 => println!("INX    B"),
        0x04 => println!("INC    B"),
        0x05 => println!("DEC    B"),
        0x06 => {println!("MVI    B,#${:02x}", buffer[pc + 1]); opbytes = 2},
        0x07 => println!("RLC"),
        0x08 => println!("NOP"),
        0x09 => println!("DAD    B"),
        0x0a => println!("LDAX   B"),
        0x0b => println!("DCX    B"),
        0x0c => println!("INC C"),
        0x0d => println!("DEC    C"),
        0x0e => {println!("LD C,{:02x}", buffer[pc + 1]); opbytes = 2},
        0x0f => println!("RRC"),

        0x10 => println!("NOP"),
        0x11 => {println!("LD DE,{:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            opbytes = 3},
        0x12 => println!("STAX   D"),
        0x13 => println!("INX D"),
        0x14 => println!("INC D"),
        0x15 => println!("DEC D"),
        0x16 => {println!("MVI    D,#${:02x}", buffer[pc + 1]); opbytes = 2},
        0x17 => println!("RAL"),
        0x18 => println!("NOP"),
        0x19 => println!("DAD    D"),
        0x1a => println!("LDAX   D"),
        0x1b => println!("DCX D"),
        0x1c => println!("INC E"),
        0x1d => println!("DEC E"),
        0x1e => {println!("MVI    E,#${:02x}", buffer[pc + 1]); opbytes = 2},
        0x1f => println!("RAR"),

        0x20 => {println!("JR NZ,{:02x}", buffer[pc + 1]); opbytes = 2},
        0x21 => {println!("LD HL,${:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            opbytes=3},
        0x22 => {println!("SHLD   ${:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            opbytes=3},
        0x23 => println!("INX    H"),
        0x24 => println!("INC    H"),
        0x25 => println!("DEC    H"),
        0x26 => {println!("MVI    H,#${:02x}", buffer[pc + 1]); opbytes=2},
        0x27 => println!("DAA"),
        0x28 => println!("NOP"),
        0x29 => println!("DAD    H"),
        0x2a => {println!("LHLD   ${:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            opbytes=3},
        0x2b => println!("DEC H"),
        0x2c => println!("INC L"),
        0x2d => println!("DEC L"),
        0x2e => {println!("MVI    L,#${:02x}", buffer[pc + 1]); opbytes = 2},
        0x2f => println!("CMA"),

        0x30 => println!("NOP"),
        0x31 => {println!("LD SP,${:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            opbytes=3},
        0x32 => println!("LD (HL-),A"),
        0x33 => println!("INX SP"),
        0x34 => println!("INC M"),
        0x35 => println!("DEC M"),
        0x36 => {println!("MVI    M,#${:02x}", buffer[pc + 1]); opbytes=2},
        0x37 => println!("STC"),
        0x38 => println!("NOP"),
        0x39 => println!("DAD    SP"),
        0x3a => {println!("LDA    ${:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            opbytes=3},
        0x3b => println!("DEC SP"),
        0x3c => println!("INC A"),
        0x3d => println!("DEC A"),
        0x3e => {println!("LD A,{:02x}", buffer[pc + 1]); opbytes = 2},
        0x3f => println!("CMC"),

        0x40 => println!("MOV B,B"),
        0x41 => println!("MOV B,C"),
        0x42 => println!("MOV B,D"),
        0x43 => println!("MOV B,E"),
        0x44 => println!("MOV B,H"),
        0x45 => println!("MOV B,L"),
        0x46 => println!("MOV B,M"),
        0x47 => println!("MOV B,A"),
        0x48 => println!("MOV C,B"),
        0x49 => println!("MOV C,C"),
        0x4a => println!("MOV C,D"),
        0x4b => println!("MOV C,E"),
        0x4c => println!("MOV C,H"),
        0x4d => println!("MOV C,L"),
        0x4e => println!("MOV C,M"),
        0x4f => println!("MOV C,A"),

        0x50 => println!("MOV D,B"),
        0x51 => println!("MOV D,C"),
        0x52 => println!("MOV D,D"),
        0x53 => println!("MOV D.E"),
        0x54 => println!("MOV D,H"),
        0x55 => println!("MOV D,L"),
        0x56 => println!("MOV D,M"),
        0x57 => println!("MOV D,A"),
        0x58 => println!("MOV E,B"),
        0x59 => println!("MOV E,C"),
        0x5a => println!("MOV E,D"),
        0x5b => println!("MOV E,E"),
        0x5c => println!("MOV E,H"),
        0x5d => println!("MOV E,L"),
        0x5e => println!("MOV E,M"),
        0x5f => println!("MOV E,A"),

        0x60 => println!("MOV H,B"),
        0x61 => println!("MOV H,C"),
        0x62 => println!("MOV H,D"),
        0x63 => println!("MOV H.E"),
        0x64 => println!("MOV H,H"),
        0x65 => println!("MOV H,L"),
        0x66 => println!("MOV H,M"),
        0x67 => println!("MOV H,A"),
        0x68 => println!("MOV L,B"),
        0x69 => println!("MOV L,C"),
        0x6a => println!("MOV L,D"),
        0x6b => println!("MOV L,E"),
        0x6c => println!("MOV L,H"),
        0x6d => println!("MOV L,L"),
        0x6e => println!("MOV L,M"),
        0x6f => println!("MOV L,A"),

        0x70 => println!("MOV M,B"),
        0x71 => println!("MOV M,C"),
        0x72 => println!("MOV M,D"),
        0x73 => println!("MOV M.E"),
        0x74 => println!("MOV M,H"),
        0x75 => println!("MOV M,L"),
        0x76 => println!("HLT"),
        0x77 => println!("MOV M,A"),
        0x78 => println!("MOV A,B"),
        0x79 => println!("MOV A,C"),
        0x7a => println!("MOV A,D"),
        0x7b => println!("MOV A,E"),
        0x7c => println!("MOV A,H"),
        0x7d => println!("MOV A,L"),
        0x7e => println!("MOV A,M"),
        0x7f => println!("MOV A,A"),

        0x80 => println!("ADD B"),
        0x81 => println!("ADD C"),
        0x82 => println!("ADD D"),
        0x83 => println!("ADD E"),
        0x84 => println!("ADD H"),
        0x85 => println!("ADD L"),
        0x86 => println!("ADD M"),
        0x87 => println!("ADD A"),
        0x88 => println!("ADC B"),
        0x89 => println!("ADC C"),
        0x8a => println!("ADC D"),
        0x8b => println!("ADC E"),
        0x8c => println!("ADC H"),
        0x8d => println!("ADC L"),
        0x8e => println!("ADC M"),
        0x8f => println!("ADC A"),

        0x90 => println!("SUB B"),
        0x91 => println!("SUB C"),
        0x92 => println!("SUB D"),
        0x93 => println!("SUB E"),
        0x94 => println!("SUB H"),
        0x95 => println!("SUB L"),
        0x96 => println!("SUB M"),
        0x97 => println!("SUB A"),
        0x98 => println!("SBB B"),
        0x99 => println!("SBB C"),
        0x9a => println!("SBB D"),
        0x9b => println!("SBB E"),
        0x9c => println!("SBB H"),
        0x9d => println!("SBB L"),
        0x9e => println!("SBB M"),
        0x9f => println!("SBB A"),

        0xa0 => println!("ANA B"),
        0xa1 => println!("ANA C"),
        0xa2 => println!("ANA D"),
        0xa3 => println!("ANA E"),
        0xa4 => println!("ANA H"),
        0xa5 => println!("ANA L"),
        0xa6 => println!("ANA M"),
        0xa7 => println!("ANA A"),
        0xa8 => println!("XOR B"),
        0xa9 => println!("XOR C"),
        0xaa => println!("XOR D"),
        0xab => println!("XOR E"),
        0xac => println!("XOR H"),
        0xad => println!("XOR L"),
        0xae => println!("XOR M"),
        0xaf => println!("XOR A"),

        0xb0 => println!("ORA B"),
        0xb1 => println!("ORA C"),
        0xb2 => println!("ORA D"),
        0xb3 => println!("ORA E"),
        0xb4 => println!("ORA H"),
        0xb5 => println!("ORA L"),
        0xb6 => println!("ORA M"),
        0xb7 => println!("ORA A"),
        0xb8 => println!("CMP B"),
        0xb9 => println!("CMP C"),
        0xba => println!("CMP D"),
        0xbb => println!("CMP E"),
        0xbc => println!("CMP H"),
        0xbd => println!("CMP L"),
        0xbe => println!("CMP M"),
        0xbf => println!("CMP A"),

        0xc0 => println!("RNZ"),
        0xc1 => println!("POP    B"),
        0xc2 => {println!("JNZ    ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xc3 => {println!("JMP    ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xc4 => {println!("CNZ    ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xc5 => println!("PUSH   B"),
        0xc6 => {println!("ADI    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xc7 => println!("RST    0"),
        0xc8 => println!("RZ"),
        0xc9 => println!("RET"),
        0xca => {println!("JZ     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xcb => {
            print!("PREFIX ");
            opbytes = 2;
            match buffer[pc + 1] {
                0x01 => println!("0x01"),
                0x02 => println!("0x02"),
                0x03 => println!("0x03"),
                0x04 => println!("0x04"),
                0x05 => println!("0x05"),
                0x06 => println!("0x06"),
                0x07 => println!("0x07"),
                0x08 => println!("0x08"),
                0x09 => println!("0x09"),
                0x0a => println!("0x0a"),
                0x0b => println!("0x0b"),
                0x0c => println!("0x0c"),
                0x0d => println!("0x0d"),
                0x0e => println!("0x0e"),
                0x0f => println!("0x0f"),

                0x11 => println!("0x11"),
                0x12 => println!("0x12"),
                0x13 => println!("0x13"),
                0x14 => println!("0x14"),
                0x15 => println!("0x15"),
                0x16 => println!("0x16"),
                0x17 => println!("0x17"),
                0x18 => println!("0x18"),
                0x19 => println!("0x19"),
                0x1a => println!("0x1a"),
                0x1b => println!("0x1b"),
                0x1c => println!("0x1c"),
                0x1d => println!("0x1d"),
                0x1e => println!("0x1e"),
                0x1f => println!("0x1f"),

                0x21 => println!("0x21"),
                0x22 => println!("0x22"),
                0x23 => println!("0x23"),
                0x24 => println!("0x24"),
                0x25 => println!("0x25"),
                0x26 => println!("0x26"),
                0x27 => println!("0x27"),
                0x28 => println!("0x28"),
                0x29 => println!("0x29"),
                0x2a => println!("0x2a"),
                0x2b => println!("0x2b"),
                0x2c => println!("0x2c"),
                0x2d => println!("0x2d"),
                0x2e => println!("0x2e"),
                0x2f => println!("0x2f"),

                0x31 => println!("0x31"),
                0x32 => println!("0x32"),
                0x33 => println!("0x33"),
                0x34 => println!("0x34"),
                0x35 => println!("0x35"),
                0x36 => println!("0x36"),
                0x37 => println!("0x37"),
                0x38 => println!("0x38"),
                0x39 => println!("0x39"),
                0x3a => println!("0x3a"),
                0x3b => println!("0x3b"),
                0x3c => println!("0x3c"),
                0x3d => println!("0x3d"),
                0x3e => println!("0x3e"),
                0x3f => println!("0x3f"),

                0x41 => println!("0x41"),
                0x42 => println!("0x42"),
                0x43 => println!("0x43"),
                0x44 => println!("0x44"),
                0x45 => println!("0x45"),
                0x46 => println!("0x46"),
                0x47 => println!("0x47"),
                0x48 => println!("0x48"),
                0x49 => println!("0x49"),
                0x4a => println!("0x4a"),
                0x4b => println!("0x4b"),
                0x4c => println!("0x4c"),
                0x4d => println!("0x4d"),
                0x4e => println!("0x4e"),
                0x4f => println!("0x4f"),

                0x51 => println!("0x51"),
                0x52 => println!("0x52"),
                0x53 => println!("0x53"),
                0x54 => println!("0x54"),
                0x55 => println!("0x55"),
                0x56 => println!("0x56"),
                0x57 => println!("0x57"),
                0x58 => println!("0x58"),
                0x59 => println!("0x59"),
                0x5a => println!("0x5a"),
                0x5b => println!("0x5b"),
                0x5c => println!("0x5c"),
                0x5d => println!("0x5d"),
                0x5e => println!("0x5e"),
                0x5f => println!("0x5f"),

                0x61 => println!("0x61"),
                0x62 => println!("0x62"),
                0x63 => println!("0x63"),
                0x64 => println!("0x64"),
                0x65 => println!("0x65"),
                0x66 => println!("0x66"),
                0x67 => println!("0x67"),
                0x68 => println!("0x68"),
                0x69 => println!("0x69"),
                0x6a => println!("0x6a"),
                0x6b => println!("0x6b"),
                0x6c => println!("0x6c"),
                0x6d => println!("0x6d"),
                0x6e => println!("0x6e"),
                0x6f => println!("0x6f"),

                0x71 => println!("0x71"),
                0x72 => println!("0x72"),
                0x73 => println!("0x73"),
                0x74 => println!("0x74"),
                0x75 => println!("0x75"),
                0x76 => println!("0x76"),
                0x77 => println!("0x77"),
                0x78 => println!("0x78"),
                0x79 => println!("0x79"),
                0x7a => println!("0x7a"),
                0x7b => println!("0x7b"),
                0x7c => println!("BIT 7,H"),
                0x7d => println!("0x7d"),
                0x7e => println!("0x7e"),
                0x7f => println!("0x7f"),

                0x81 => println!("0x81"),
                0x82 => println!("0x82"),
                0x83 => println!("0x83"),
                0x84 => println!("0x84"),
                0x85 => println!("0x85"),
                0x86 => println!("0x86"),
                0x87 => println!("0x87"),
                0x88 => println!("0x88"),
                0x89 => println!("0x89"),
                0x8a => println!("0x8a"),
                0x8b => println!("0x8b"),
                0x8c => println!("0x8c"),
                0x8d => println!("0x8d"),
                0x8e => println!("0x8e"),
                0x8f => println!("0x8f"),

                0x91 => println!("0x91"),
                0x92 => println!("0x92"),
                0x93 => println!("0x93"),
                0x94 => println!("0x94"),
                0x95 => println!("0x95"),
                0x96 => println!("0x96"),
                0x97 => println!("0x97"),
                0x98 => println!("0x98"),
                0x99 => println!("0x99"),
                0x9a => println!("0x9a"),
                0x9b => println!("0x9b"),
                0x9c => println!("0x9c"),
                0x9d => println!("0x9d"),
                0x9e => println!("0x9e"),
                0x9f => println!("0x9f"),

                0xa1 => println!("0xa1"),
                0xa2 => println!("0xa2"),
                0xa3 => println!("0xa3"),
                0xa4 => println!("0xa4"),
                0xa5 => println!("0xa5"),
                0xa6 => println!("0xa6"),
                0xa7 => println!("0xa7"),
                0xa8 => println!("0xa8"),
                0xa9 => println!("0xa9"),
                0xaa => println!("0xaa"),
                0xab => println!("0xab"),
                0xac => println!("0xac"),
                0xad => println!("0xad"),
                0xae => println!("0xae"),
                0xaf => println!("0xaf"),

                0xb1 => println!("0xb1"),
                0xb2 => println!("0xb2"),
                0xb3 => println!("0xb3"),
                0xb4 => println!("0xb4"),
                0xb5 => println!("0xb5"),
                0xb6 => println!("0xb6"),
                0xb7 => println!("0xb7"),
                0xb8 => println!("0xb8"),
                0xb9 => println!("0xb9"),
                0xba => println!("0xba"),
                0xbb => println!("0xbb"),
                0xbc => println!("0xbc"),
                0xbd => println!("0xbd"),
                0xbe => println!("0xbe"),
                0xbf => println!("0xbf"),

                0xc1 => println!("0xc1"),
                0xc2 => println!("0xc2"),
                0xc3 => println!("0xc3"),
                0xc4 => println!("0xc4"),
                0xc5 => println!("0xc5"),
                0xc6 => println!("0xc6"),
                0xc7 => println!("0xc7"),
                0xc8 => println!("0xc8"),
                0xc9 => println!("0xc9"),
                0xca => println!("0xca"),
                0xcb => println!("0xcb"),
                0xcc => println!("0xcc"),
                0xcd => println!("0xcd"),
                0xce => println!("0xce"),
                0xcf => println!("0xcf"),

                0xd1 => println!("0xd1"),
                0xd2 => println!("0xd2"),
                0xd3 => println!("0xd3"),
                0xd4 => println!("0xd4"),
                0xd5 => println!("0xd5"),
                0xd6 => println!("0xd6"),
                0xd7 => println!("0xd7"),
                0xd8 => println!("0xd8"),
                0xd9 => println!("0xd9"),
                0xda => println!("0xda"),
                0xdb => println!("0xdb"),
                0xdc => println!("0xdc"),
                0xdd => println!("0xdd"),
                0xde => println!("0xde"),
                0xdf => println!("0xdf"),

                0xe1 => println!("0xe1"),
                0xe2 => println!("0xe2"),
                0xe3 => println!("0xe3"),
                0xe4 => println!("0xe4"),
                0xe5 => println!("0xe5"),
                0xe6 => println!("0xe6"),
                0xe7 => println!("0xe7"),
                0xe8 => println!("0xe8"),
                0xe9 => println!("0xe9"),
                0xea => println!("0xea"),
                0xeb => println!("0xeb"),
                0xec => println!("0xec"),
                0xed => println!("0xed"),
                0xee => println!("0xee"),
                0xef => println!("0xef"),

                0xf1 => println!("0xf1"),
                0xf2 => println!("0xf2"),
                0xf3 => println!("0xf3"),
                0xf4 => println!("0xf4"),
                0xf5 => println!("0xf5"),
                0xf6 => println!("0xf6"),
                0xf7 => println!("0xf7"),
                0xf8 => println!("0xf8"),
                0xf9 => println!("0xf9"),
                0xfa => println!("0xfa"),
                0xfb => println!("0xfb"),
                0xfc => println!("0xfc"),
                0xfd => println!("0xfd"),
                0xfe => println!("0xfe"),
                0xff => println!("0xff"),
                _ => unreachable!()
            }
        },
        0xcc => {println!("CZ     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xcd => {println!("CALL   ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xce => {println!("ACI    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xcf => println!("RST    1"),

        0xd0 => println!("RNC"),
        0xd1 => println!("POP    D"),
        0xd2 => {println!("JNC    ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xd3 => {println!("OUT    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xd4 => {println!("CNC    ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xd5 => println!("PUSH   D"),
        0xd6 => {println!("SUI    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xd7 => println!("RST    2"),
        0xd8 => println!("RC"),
        0xd9 => println!("RET"),
        0xda => {println!("JC     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xdb => {println!("IN     #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xdc => {println!("CC     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xdd => {println!("CALL   ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xde => {println!("SBI    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xdf => println!("RST    3"),

        0xe0 => {println!("LDH (0xff{:02x}),A",buffer[pc + 1])},
        0xe1 => println!("POP    H"),
        0xe2 => println!("LD (C),A"),
        0xe3 => println!("XTHL"),
        0xe4 => {println!("CPO    ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xe5 => println!("PUSH   H"),
        0xe6 => {println!("ANI    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xe7 => println!("RST    4"),
        0xe8 => println!("RPE"),
        0xe9 => println!("PCHL"),
        0xea => {println!("JPE    ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xeb => println!("XCHG"),
        0xec => {println!("CPE     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xed => {println!("CALL   ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xee => {println!("XRI    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xef => println!("RST    5"),

        0xf0 => println!("RP"),
        0xf1 => println!("POP    PSW"),
        0xf2 => {println!("JP     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xf3 => println!("DI"),
        0xf4 => {println!("CP     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xf5 => println!("PUSH   PSW"),
        0xf6 => {println!("ORI    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xf7 => println!("RST    6"),
        0xf8 => println!("RM"),
        0xf9 => println!("SPHL"),
        0xfa => {println!("JM     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xfb => println!("EI"),
        0xfc => {println!("CM     ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xfd => {println!("CALL   ${:02x}{:02x}",buffer[pc + 2],buffer[pc + 1]);
            opbytes = 3},
        0xfe => {println!("CPI    #${:02x}",buffer[pc + 1]); opbytes = 2},
        0xff => println!("RST    7"),
    }

    opbytes
}

pub fn hexdump(buffer: Vec<u8>) {
    for (i, v) in buffer.iter().enumerate() {
        if i % 16 == 0 {
            print!("{:04x} ", i);
        }
        print!("{:02x} ", v);
        if i % 16 == 15 {
            print!("\n");
        }
    }
}

pub fn hexdump_memory(state: sm83cpu::StateSM83) {
    for (i, v) in state.memory.iter().enumerate() {
        if i % 16 == 0 {
            print!("{:04x} ", i);
        }
        print!("{:02x} ", v);
        if i % 16 == 15 {
            print!("\n");
        }
    }
    println!();
}
