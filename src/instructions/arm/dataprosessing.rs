use crate::errors::ParseError;
use crate::instructions::{split_with_mask, Register, ShifterOperand};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataProssessingInstruction {
    ///Add with Carry. See ADC on page A4-4.
    ADC {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Add. See ADD on page A4-6.
    ADD {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Logical AND. See AND on page A4-8.
    AND {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Logical Bit Clear. See BIC on page A4-12.
    BIC {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Compare Negative. See CMN on page A4-26.
    CMN {
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Compare. See CMP on page A4-28.
    CMP {
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Logical EOR. See EOR on page A4-32.
    EOR {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Move. See MOV on page A4-68.
    MOV {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Move Not. See MVN on page A4-82.
    MVN {
        destination: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Logical OR. See ORR on page A4-84.
    ORR {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Reverse Subtract. See RSB on page A4-115.
    RSB {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Reverse Subtract with Carry. See RSC on page A4-117.
    RSC {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Subtract with Carry. See SBC on page A4-125.
    SBC {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Subtract. See SUB on page A4-208.
    SUB {
        destination: Register,
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Test Equivalence. See TEQ on page A4-228.
    TEQ {
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
    ///Test. See TST on page A4-230.
    TST {
        first_operand: Register,
        s: bool,
        shifter: ShifterOperand,
    },
}
impl TryFrom<u32> for DataProssessingInstruction {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let (opcode, rest) = split_with_mask(value, 0b1111 << 20);
        let (rn, rest) = split_with_mask(rest, 0b1111 << 15);
        let rn = ((rn >> 16) as u8).try_into()?;
        let s = ((rest >> 20) & 1) == 1;
        use DataProssessingInstruction::*;
        Ok(match opcode >> 20 {
            0b0101 => {
                let (rd, rest) = split_with_mask(rest, 0b1111 << 11);
                let rd = ((rd >> 12) as u8).try_into()?;
                let shifter = rest.try_into()?;
                ADC {
                    destination: rd,
                    first_operand: rn,
                    s,
                    shifter,
                }
            }
            0b0100 => {
                let (rd, rest) = split_with_mask(rest, 0b1111 << 11);
                let rd = ((rd >> 12) as u8).try_into()?;
                let shifter = rest.try_into()?;
                ADD {
                    destination: rd,
                    first_operand: rn,
                    s,
                    shifter,
                }
            }
            0b0000 => {
                let (rd, rest) = split_with_mask(rest, 0b1111 << 11);
                let rd = ((rd >> 12) as u8).try_into()?;
                let shifter = rest.try_into()?;
                AND {
                    destination: rd,
                    first_operand: rn,
                    s,
                    shifter,
                }
            }
            0b0000 => {
                let (rd, rest) = split_with_mask(rest, 0b1111 << 11);
                let rd = ((rd >> 12) as u8).try_into()?;
                let shifter = rest.try_into()?;
                BIC {
                    destination: rd,
                    first_operand: rn,
                    s,
                    shifter,
                }
            }
            bin => todo!("unimpemented dataprocessing opcode {:b}", bin),
        })
    }
}
