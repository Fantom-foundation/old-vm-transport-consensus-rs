type Gas = u32;

/// Opcodes supported by the Ethereum VM. https://github.com/trailofbits/evm-opcodes is a good reference for them.
#[derive(PartialEq, Hash, Debug)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SLOAD,
    STORE,
    MLOAD,
    MSTORE,
    MSTORE8,
    MSIZE,
    PUSH(usize),
    DUP(usize),
    SWAP(usize),
    LOG(usize),
    INVALID,
    SUICIDE,
}

impl<'a> From<&'a u8> for Opcode {
    fn from(bytes: &u8) -> Self {
        match bytes {
            0x00 => Opcode::STOP,
            0x01 => Opcode::ADD,
            0x02 => Opcode::MUL,
            0x03 => Opcode::SUB,
            0x04 => Opcode::DIV,
            0x05 => Opcode::SDIV,
            0x06 => Opcode::MOD,
            0x07 => Opcode::SMOD,
            0x08 => Opcode::ADDMOD,
            0x09 => Opcode::MULMOD,
            0x0a => Opcode::EXP,
            0x0b => Opcode::SIGNEXTEND,
            0x10 => Opcode::LT,
            0x11 => Opcode::GT,
            0x12 => Opcode::SLT,
            0x13 => Opcode::SGT,
            0x14 => Opcode::EQ,
            0x15 => Opcode::ISZERO,
            0x16 => Opcode::AND,
            0x17 => Opcode::OR,
            0x18 => Opcode::XOR,
            0x19 => Opcode::NOT,
            0x1a => Opcode::BYTE,
            0x54 => Opcode::SLOAD,
            0x55 => Opcode::STORE,
            0x51 => Opcode::MLOAD,
            0x52 => Opcode::MSTORE,
            0x53 => Opcode::MSTORE8,
            0x59 => Opcode::MSIZE,
            0x60 => Opcode::PUSH(1),
            0x61 => Opcode::PUSH(2),
            0x62 => Opcode::PUSH(3),
            0x63 => Opcode::PUSH(4),
            0x64 => Opcode::PUSH(5),
            0x65 => Opcode::PUSH(6),
            0x66 => Opcode::PUSH(7),
            0x67 => Opcode::PUSH(8),
            0x68 => Opcode::PUSH(9),
            0x69 => Opcode::PUSH(10),
            0x6a => Opcode::PUSH(11),
            0x6b => Opcode::PUSH(12),
            0x6c => Opcode::PUSH(13),
            0x6d => Opcode::PUSH(14),
            0x6e => Opcode::PUSH(15),
            0x6f => Opcode::PUSH(16),
            0x70 => Opcode::PUSH(17),
            0x71 => Opcode::PUSH(18),
            0x72 => Opcode::PUSH(19),
            0x73 => Opcode::PUSH(20),
            0x74 => Opcode::PUSH(21),
            0x75 => Opcode::PUSH(22),
            0x76 => Opcode::PUSH(23),
            0x77 => Opcode::PUSH(24),
            0x78 => Opcode::PUSH(25),
            0x79 => Opcode::PUSH(26),
            0x7a => Opcode::PUSH(27),
            0x7b => Opcode::PUSH(28),
            0x7c => Opcode::PUSH(29),
            0x7d => Opcode::PUSH(30),
            0x7e => Opcode::PUSH(31),
            0x7f => Opcode::PUSH(32),

            0x80 => Opcode::DUP(1),
            0x81 => Opcode::DUP(2),
            0x82 => Opcode::DUP(3),
            0x83 => Opcode::DUP(4),
            0x84 => Opcode::DUP(5),
            0x85 => Opcode::DUP(6),
            0x86 => Opcode::DUP(7),
            0x87 => Opcode::DUP(8),
            0x88 => Opcode::DUP(9),
            0x89 => Opcode::DUP(10),
            0x8a => Opcode::DUP(11),
            0x8b => Opcode::DUP(12),
            0x8c => Opcode::DUP(13),
            0x8d => Opcode::DUP(14),
            0x8e => Opcode::DUP(15),
            0x8f => Opcode::DUP(16),

            0x90 => Opcode::SWAP(1),
            0x91 => Opcode::SWAP(2),
            0x92 => Opcode::SWAP(3),
            0x93 => Opcode::SWAP(4),
            0x94 => Opcode::SWAP(5),
            0x95 => Opcode::SWAP(6),
            0x96 => Opcode::SWAP(7),
            0x97 => Opcode::SWAP(8),
            0x98 => Opcode::SWAP(9),
            0x99 => Opcode::SWAP(10),
            0x9a => Opcode::SWAP(11),
            0x9b => Opcode::SWAP(12),
            0x9c => Opcode::SWAP(13),
            0x9d => Opcode::SWAP(14),
            0x9e => Opcode::SWAP(15),
            0x9f => Opcode::SWAP(16),

            0xa0 => Opcode::LOG(0),
            0xa1 => Opcode::LOG(1),
            0xa2 => Opcode::LOG(2),
            0xa3 => Opcode::LOG(3),
            0xa4 => Opcode::LOG(4),

            0xfe => Opcode::INVALID,
            0xff => Opcode::SUICIDE,
            _ => Opcode::INVALID,
        }
    }
}
