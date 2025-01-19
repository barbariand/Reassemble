use crate::errors::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptiongeneratingInstruction {
    ///Breakpoint. See BKPT on page A4-14.
    BKPT(u16),
    ///Software Interrupt. See SWI on page A4-210.
    SWI(u32),
}
impl ExceptiongeneratingInstruction {
    pub fn new(rest: u32) -> Result<Self, ParseError> {
        todo!("Exception generating instructions")
    }
}
