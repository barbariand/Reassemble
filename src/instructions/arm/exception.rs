#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptiongeneratingInstruction {
    ///Breakpoint. See BKPT on page A4-14.
    BKPT(u16),
    ///Software Interrupt. See SWI on page A4-210.
    SWI(u32),
}
