use crate::errors::ParseError;
use crate::instructions::{CRegister, Coprocessor, Register};

use super::AddressingMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnconditionalInstruction {
    BLX(i32),
    LDC {
        coprocessor: Coprocessor,
        long_load: bool,
        destination: CRegister,
        addressing_mode: AddressingMode,
    },
    MCR {
        coprocessor: Coprocessor,
        value: Register,
        destination: CRegister,
        additional_destination: CRegister,
        opcode_1: u8,
        opcode_2: u8,
    },
    MRC{
        coprocessor: Coprocessor,
        value:Register,
        destination: CRegister,
        additional_destination: CRegister,
        opcode_1: u8,
        opcode_2: u8,
    },
    MRRC{
        coprocessor: Coprocessor,
        opcode: u8,
        first_register:Register,
        second_register:Register,
        destination:CRegister,
    },
    ///Pre load data. See PLD on page A4-90.
    PLD{
        addressing_mode:AddressingMode,
    },
}

impl TryFrom<u32> for UnconditionalInstruction {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        todo!()
    }
}
