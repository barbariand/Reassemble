use crate::errors::ParseError;
use crate::instructions::split_with_mask;

use arithmetic::AritmeticInstruction;
use branch::BranchInstruction;
use coprocessor::CoprocessorInstruction;
use dataprosessing::DataProssessingInstruction;
use exception::ExceptiongeneratingInstruction;
use loadandstore::LoadAndStoreInstruction;
use multiply::MultiplyInstruction;
use semaphore::SemaphoreInstruction;
use unconditional::UnconditionalInstruction;

use self::register_access_instructions::RegisterAccessInstruction;
pub trait TryArmInstruction: Sized {
    const MASK: u32;
    fn try_from(value: u32) -> Result<Self, ParseError>;
    const fn split_mask(value: u32) -> (u32,u32) {
        let first = value & Self::MASK;
        let second = value & !Self::MASK;
        (first, second)
    }
}
mod adresssing;
pub mod arithmetic;
pub mod branch;
pub mod coprocessor;
pub mod dataprosessing;
pub mod exception;
pub mod loadandstore;
pub mod multiply;
mod register_access_instructions;
pub mod semaphore;
pub mod unconditional;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostIndexedAddressingMode {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressingMode {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmInstruction {
    /// 0000 EQ
    Equal(PartialArmInstruction),
    /// 0001 NE
    NotEqual(PartialArmInstruction),
    /// 0010 CS/HS
    CarrySet(PartialArmInstruction),
    /// 0011 CC/LO
    CarryClear(PartialArmInstruction),
    /// 0100 MI
    Minus(PartialArmInstruction),
    /// 0101 PL
    Plus(PartialArmInstruction),
    /// 0110 VS
    Overflow(PartialArmInstruction),
    /// 0111 VC
    NoOverflow(PartialArmInstruction),
    /// 1000 HI
    Higher(PartialArmInstruction),
    /// 1001 LS
    LowerOrSame(PartialArmInstruction),
    /// 1010 GE
    SignedGreaterOrEqual(PartialArmInstruction),
    /// 1011 LT
    SignedLessThan(PartialArmInstruction),
    /// 1100 GT
    SignedGreaterThan(PartialArmInstruction),
    /// 1101 LE
    SignedLessThanOrEqual(PartialArmInstruction),
    /// 1110 AL
    Allways(PartialArmInstruction),
    /// 1111 UC
    Unconditional(UnconditionalInstruction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartialArmInstruction {
    // Branch Instructions
    Branch(BranchInstruction),
    DataProssessing(DataProssessingInstruction),
    Multiply(MultiplyInstruction),
    Aritmetic(AritmeticInstruction),
    LoadAndStore(LoadAndStoreInstruction),
    Semaphore(SemaphoreInstruction),
    Exceptiongenerating(ExceptiongeneratingInstruction),
    Coprocessor(CoprocessorInstruction),
    RegisterAccess(RegisterAccessInstruction),
}
impl TryFrom<u32> for PartialArmInstruction {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<u32> for ArmInstruction {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let (cond, rest) = split_with_mask(value, 0b1111 << 28);
        use ArmInstruction::*;
        Ok(match cond {
            (0) => {
                let inst: PartialArmInstruction = rest.try_into()?;
                Equal(inst)
            }

            (1) => {
                let inst = rest.try_into()?;
                NotEqual(inst)
            }
            (2) => {
                let inst = rest.try_into()?;
                CarrySet(inst)
            }
            (3) => {
                let inst = rest.try_into()?;
                CarryClear(inst)
            }
            (4) => {
                let inst = rest.try_into()?;
                Minus(inst)
            }
            (5) => {
                let inst = rest.try_into()?;
                Plus(inst)
            }
            (6) => {
                let inst = rest.try_into()?;
                Overflow(inst)
            }
            (7) => {
                let inst = rest.try_into()?;
                NoOverflow(inst)
            }
            (8) => {
                let inst = rest.try_into()?;
                Higher(inst)
            }
            (9) => {
                let inst = rest.try_into()?;
                LowerOrSame(inst)
            }
            (10) => {
                let inst = rest.try_into()?;
                SignedGreaterOrEqual(inst)
            }
            (11) => {
                let inst = rest.try_into()?;
                SignedLessThan(inst)
            }
            (12) => {
                let inst = rest.try_into()?;
                SignedGreaterThan(inst)
            }
            (13) => {
                let inst = rest.try_into()?;
                SignedLessThanOrEqual(inst)
            }
            (14) => {
                let inst = rest.try_into()?;
                Allways(inst)
            }
            (15) => {
                let inst = rest.try_into()?;
                Unconditional(inst)
            }
            (16_u32..=u32::MAX) => {
                unreachable!("Masking in cond is wrong, got {} expected < 16", cond)
            }
        })
    }
}
