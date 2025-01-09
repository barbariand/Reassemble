use crate::instructions::{PSRFlags, Register};

pub enum RegisterAccessInstruction {
    ///Move PSR to General-purpose Register. See MRS on page A4-74.
    MRS{
        destination:Register,
        is_spsr:bool,
    },
    ///Move General-purpose Register to PSR. See MSR on page A4-76.
    MSR{
        flags:PSRFlags,
        destination:Register,
        is_spsr:bool,
    },
    ///Change Processor State. Changes one or more of the processor mode and interrupt enable bits of the CPSR, without changing the other CPSR bits. See CPS on page A4-29.
    CPS,
    ///Modifies the CPSR endianness, E, bit, without changing any other bits in the CPSR. See SETEND on page A4-129.
    SETEND,
}
