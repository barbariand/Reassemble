use crate::instructions::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemaphoreInstruction {
    ///Swap. See SWP on page A4-212.
    SWP {
        destination: Register,
        value: Register,
        mem: Register,
    },
    ///Swap Byte. See SWPB on page A4-214.
    SWPB {
        destination: Register,
        value: Register,
        mem: Register,
    },
}
