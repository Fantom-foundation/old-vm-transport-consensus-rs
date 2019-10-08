use crate::instruction::Instruction;
use failure::Error;

type Gas = u32;

/// Opcodes supported by the Ethereum VM. https://github.com/trailofbits/evm-opcodes is a good
/// reference for them.
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
    SHA3,
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    MSIZE,
    GAS,
    JUMPDEST,
    PUSH(u64),
    DUP(u64),
    SWAP(u64),
    LOG(u64),
    INVALID,
    SUICIDE,
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
}

impl Instruction for Opcode {
    fn size(&self) -> Result<usize, Error> {
        Ok(match self {
            Opcode::PUSH(_) | Opcode::DUP(_) | Opcode::SWAP(_) | Opcode::LOG(_) => 9,
            _ => 1,
        })
    }
    fn get_cycles(&self) -> Result<usize, Error> {
        Ok(1)
    }
}

// Converts a byte into an Opcode for convenience
impl<'a> From<&'a u8> for Opcode {
    fn from(bytes: &u8) -> Self {
        match bytes {
            // Math, Comparison, and Bitwise Logic Operations
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

            // Cryptographic Operations
            0x20 => Opcode::SHA3,

            // Environmental information Operations
            0x30 => Opcode::ADDRESS,
            0x31 => Opcode::BALANCE,
            0x32 => Opcode::ORIGIN,
            0x33 => Opcode::CALLER,
            0x34 => Opcode::CALLVALUE,
            0x35 => Opcode::CALLDATALOAD,
            0x36 => Opcode::CALLDATASIZE,
            0x37 => Opcode::CALLDATACOPY,
            0x38 => Opcode::CODESIZE,
            0x39 => Opcode::CODECOPY,
            0x3a => Opcode::GASPRICE,
            0x3b => Opcode::EXTCODESIZE,
            0x3c => Opcode::EXTCODECOPY,
            0x3d => Opcode::RETURNDATASIZE,
            0x3e => Opcode::RETURNDATACOPY,

            // Block data functions
            0x40 => Opcode::BLOCKHASH,
            0x41 => Opcode::COINBASE,
            0x42 => Opcode::TIMESTAMP,
            0x43 => Opcode::NUMBER,
            0x44 => Opcode::DIFFICULTY,
            0x45 => Opcode::GASLIMIT,

            // Stack, memory, storage, and flow operations
            0x50 => Opcode::POP,
            0x51 => Opcode::MLOAD,
            0x52 => Opcode::MSTORE,
            0x53 => Opcode::MSTORE8,
            0x54 => Opcode::SLOAD,
            0x55 => Opcode::SSTORE,
            0x56 => Opcode::JUMP,
            0x57 => Opcode::JUMPI,
            0x58 => Opcode::PC,
            0x59 => Opcode::MSIZE,
            0x5a => Opcode::GAS,
            0x5b => Opcode::JUMPDEST,

            // Push operations
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

            // Duplication operations
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

            // Swap operations
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

            // Logging operations
            0xa0 => Opcode::LOG(0),
            0xa1 => Opcode::LOG(1),
            0xa2 => Opcode::LOG(2),
            0xa3 => Opcode::LOG(3),
            0xa4 => Opcode::LOG(4),

            // System operations
            0xf0 => Opcode::CREATE,
            0xf1 => Opcode::CALL,
            0xf2 => Opcode::CALLCODE,
            0xf3 => Opcode::RETURN,
            0xf4 => Opcode::DELEGATECALL,
            0xfe => Opcode::INVALID,
            0xff => Opcode::SUICIDE,
            _ => Opcode::INVALID,
        }
    }
}
