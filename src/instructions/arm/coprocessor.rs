use crate::instructions::{CRegister, Coprocessor, Register};

use super::AddressingMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoprocessorInstruction {
    ///Coprocessor Data Operations. See CDP on page A4-23.
    CDP {
        coprocessor: Coprocessor,
        destination: CRegister,
        first_operand: CRegister,
        second_operand: CRegister,
        opcode_1: u8,
        opcode_2: u8,
    },
    ///Load Coprocessor Register. See LDC on page A4-34.
    LDC {
        coprocessor: Coprocessor,
        long_load: bool,
        destination: CRegister,
        addressing_mode: AddressingMode,
    },
    ///Move to Coprocessor from ARM Register. See MCR on page A4-62.
    MCR {
        coprocessor: Coprocessor,
        value: Register,
        destination: CRegister,
        additional_destination: CRegister,
        opcode_1: u8,
        opcode_2: u8,
    },
    ///Move to Coprocessor from two ARM Registers. See MCRR on page A4-64.
    MCRR {
        coprocessor: Coprocessor,
        opcode: u8,
        first_register: Register,
        second_register: Register,
        destination: CRegister,
    },
    ///Move to ARM Register from Coprocessor. See MRC on page A4-70.
    MRC {
        coprocessor: Coprocessor,
        value: Register,
        destination: CRegister,
        additional_destination: CRegister,
        opcode_1: u8,
        opcode_2: u8,
    },
    ///Move to two ARM Registers from Coprocessor. See MRRC on page A4-72.
    MRRC {
        coprocessor: Coprocessor,
        opcode: u8,
        first_register: Register,
        second_register: Register,
        destination: CRegister,
    },
    ///Store Coprocessor Register. See STC on page A4-186.
    STC {
        l: bool,

        coprocessor: Coprocessor,
        soruce: CRegister,
        addressing_mode: AddressingMode,
    },
}
