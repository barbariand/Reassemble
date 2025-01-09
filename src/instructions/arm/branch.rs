use crate::instructions::{Register, RelativeAdress};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchInstruction {
    ///Branch. See B on page A4-10.
    B(RelativeAdress),
    ///Branch with Link. See BL on page A4-10.
    BL(RelativeAdress),
    ///Branch with Link and Exchange. See BLX (1) on page A4-16 and BLX (2) on page A4-18.
    BLX(Register),
    ///Branch and Exchange Instruction Set. See BX on page A4-20.
    BX(Register),
}
