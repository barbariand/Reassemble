use ux::u4;

use crate::instructions::{PSRFlags, Register, ShifterOperand};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterAccessInstruction {
    ///Move PSR to General-purpose Register. See MRS on page A4-74.
    MRS {
        destination: Register,
        is_spsr: bool,
    },
    ///Move General-purpose Register to PSR. See MSR on page A4-76.
    MSR {
        flags: PSRFlags,
        shifter_operand: ShifterOperand,
        is_spsr: bool,
    },
    ///Change Processor State. Changes one or more of the processor mode and interrupt enable bits of the CPSR, without changing the other CPSR bits. See CPS on page A4-29.
    CPS {
        flags: Option<IFlags>,
        mode: Option<u4>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IFlags {
    a: bool,
    i: bool,
    f: bool,
}
