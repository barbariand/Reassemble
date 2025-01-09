#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemaphoreInstruction {
    ///Swap. See SWP on page A4-212.
    SWP,
    ///Swap Byte. See SWPB on page A4-214.
    SWPB,
}
