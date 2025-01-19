use crate::errors::ParseError;
use crate::instructions::{check_bit, CRegister, Coprocessor, Register};

use super::AddressingMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoprocessorInstruction {
    ///Coprocessor Data Operations. See CDP on page A4-23.
    CDP(CDP),
    ///Load Coprocessor Register. See LDC on page A4-34.
    LDC(LDC),
    ///Move to Coprocessor from ARM Register. See MCR on page A4-62.
    MCR(MCR),
    ///Move to Coprocessor from two ARM Registers. See MCRR on page A4-64.
    MCRR(MCRR),
    ///Move to ARM Register from Coprocessor. See MRC on page A4-70.
    MRC(MRC),
    ///Move to two ARM Registers from Coprocessor. See MRRC on page A4-72.
    MRRC(MRRC),
    ///Store Coprocessor Register. See STC on page A4-186.
    STC(STC),
}
impl CoprocessorInstruction {
    pub fn new(value: u32, is_transfer: bool) -> Result<CoprocessorInstruction, ParseError> {
        match is_transfer {
            true => {
                let mrc = check_bit(value, 20);
                match mrc {
                    _ => todo!("MCR MRC"),
                }
            }
            false => {
                todo!("coprocessor {:032b}, {}", value, is_transfer);
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CDP {
    coprocessor: Coprocessor,
    destination: CRegister,
    first_operand: CRegister,
    second_operand: CRegister,
    opcode_1: u8,
    opcode_2: u8,
}
impl CDP {
    fn new(value: u32) -> Result<Self, ParseError> {
        todo!()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LDC {
    coprocessor: Coprocessor,
    long_load: bool,
    destination: CRegister,
    addressing_mode: AddressingMode,
}
///Move to Coprocessor from ARM Register. See MCR on page A4-62.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCR {
    coprocessor: Coprocessor,
    value: Register,
    destination: CRegister,
    additional_destination: CRegister,
    opcode_1: u8,
    opcode_2: u8,
}
///Move to Coprocessor from two ARM Registers. See MCRR on page A4-64.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCRR {
    coprocessor: Coprocessor,
    opcode: u8,
    first_register: Register,
    second_register: Register,
    destination: CRegister,
}
///Move to ARM Register from Coprocessor. See MRC on page A4-70.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MRC {
    coprocessor: Coprocessor,
    value: Register,
    destination: CRegister,
    additional_destination: CRegister,
    opcode_1: u8,
    opcode_2: u8,
}
///Move to two ARM Registers from Coprocessor. See MRRC on page A4-72.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MRRC {
    coprocessor: Coprocessor,
    opcode: u8,
    first_register: Register,
    second_register: Register,
    destination: CRegister,
}
///Store Coprocessor Register. See STC on page A4-186.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct STC {
    l: bool,
    coprocessor: Coprocessor,
    soruce: CRegister,
    addressing_mode: AddressingMode,
}
