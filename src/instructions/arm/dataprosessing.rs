use crate::errors::ParseError;
use crate::instructions::{split_with_mask, Register, ShifterOperand};
pub enum Test {
    TEST,
    TEST2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataProssessingInstruction {
    ///Add with Carry. See ADC on page A4-4.
    ADC(GenericDataInstruction),
    ///Add. See ADD on page A4-6.
    ADD(GenericDataInstruction),
    ///Logical AND. See AND on page A4-8.
    AND(GenericDataInstruction),
    ///Logical Bit Clear. See BIC on page A4-12.
    BIC(GenericDataInstruction),
    ///Compare Negative. See CMN on page A4-26.
    CMN(NoDestinationDataInstruction),
    ///Compare. See CMP on page A4-28.
    CMP(NoDestinationDataInstruction),
    ///Logical EOR. See EOR on page A4-32.
    EOR(GenericDataInstruction),
    ///Move. See MOV on page A4-68.
    MOV(MOVLikeDataInstruction),
    ///Move Not. See MVN on page A4-82.
    MVN(MOVLikeDataInstruction),
    ///Logical OR. See ORR on page A4-84.
    ORR(GenericDataInstruction),
    ///Reverse Subtract. See RSB on page A4-115.
    RSB(GenericDataInstruction),
    ///Reverse Subtract with Carry. See RSC on page A4-117.
    RSC(GenericDataInstruction),
    ///Subtract with Carry. See SBC on page A4-125.
    SBC(GenericDataInstruction),
    ///Subtract. See SUB on page A4-208.
    SUB(GenericDataInstruction),
    ///Test Equivalence. See TEQ on page A4-228.
    TEQ(NoDestinationDataInstruction),
    ///Test. See TST on page A4-230.
    TST(NoDestinationDataInstruction),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericDataInstruction {
    destination: Register,
    first_operand: Register,
    s: bool,
    shifter: ShifterOperand,
}
impl GenericDataInstruction {
    fn new(value: u32, rn: Register) -> Result<Self, ParseError> {
        let s = ((value >> 20) & 1) == 1;
        let (rd, rest) = split_with_mask(value, 0b1111 << 11);
        let rd = ((rd >> 12) as u8).try_into()?;
        let shifter = rest.try_into()?;
        Ok(Self {
            destination: rd,
            first_operand: rn,
            s,
            shifter,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoDestinationDataInstruction {
    first_operand: Register,
    s: bool,
    shifter: ShifterOperand,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MOVLikeDataInstruction {
    destination: Register,
    s: bool,
    shifter: ShifterOperand,
}
impl NoDestinationDataInstruction {
    fn new(value: u32, rn: Register) -> Result<Self, ParseError> {
        todo!("")
    }
}
impl TryFrom<u32> for DataProssessingInstruction {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let (opcode, rest) = split_with_mask(value, 0b1111 << 20);
        let (rn, rest) = split_with_mask(rest, 0b1111 << 15);
        let rn = ((rn >> 16) as u8).try_into()?;
        use DataProssessingInstruction::*;
        Ok(match opcode >> 20 {
            0b0101 => ADC(GenericDataInstruction::new(rest, rn)?),
            0b0100 => ADD(GenericDataInstruction::new(rest, rn)?),
            0b0000 => AND(GenericDataInstruction::new(rest, rn)?),
            0b1110 => BIC(GenericDataInstruction::new(rest, rn)?),
            bin => todo!("unimpemented dataprocessing opcode {:b}", bin),
        })
    }
}
