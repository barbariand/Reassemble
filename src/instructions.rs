#![allow(unused)]
//! using https://documentation-service.arm.com/static/5f8dacc8f86e16515cdb865a?token=#E3.Ciaeiijh
pub mod arm;

use crate::errors::{DisasemblerError, ParseError};
use bitflags::bitflags;
use std::mem;
use std::ops::BitOr;
use ux::u4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PSRFlags {
    c: bool,
    x: bool,
    s: bool,
    f: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterList(u16);
bitflags! {
    impl RegisterList:u16{
        const REGISTER0 = 0b1<<0;
        const REGISTER1 = 0b1<<1;
        const REGISTER2 = 0b1<<2;
        const REGISTER3 = 0b1<<3;
        const REGISTER4 = 0b1<<4;
        const REGISTER5 = 0b1<<5;
        const REGISTER6 = 0b1<<6;
        const REGISTER7 = 0b1<<7;
        const REGISTER8 = 0b1<<8;
        const REGISTER9 = 0b1<<9;
        const REGISTER11 = 0b1<<11;
        const REGISTER12 = 0b1<<12;
        const REGISTER13 = 0b1<<13;
        const REGISTER14 = 0b1<<14;
        const REGISTER15 = 0b1<<15;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coprocessor {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}
impl TryFrom<u8> for Coprocessor {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, ParseError> {
        use Coprocessor::*;
        Ok(match value {
            0 => P0,
            1 => P1,
            2 => P2,
            3 => P3,
            4 => P4,
            5 => P5,
            6 => P6,
            7 => P7,
            8 => P8,
            9 => P9,
            10 => P10,
            11 => P11,
            12 => P12,
            13 => P13,
            14 => P14,
            15 => P15,
            e => {
                return Err(ParseError::InvalidMask {
                    invalid_set_bytes: { (e & !(0b1111)) as u32 },
                })
            }
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
impl TryFrom<u8> for Register {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, ParseError> {
        use Register::*;
        Ok(match value {
            0 => R0,
            1 => R1,
            2 => R2,
            3 => R3,
            4 => R4,
            5 => R5,
            6 => R6,
            7 => R7,
            8 => R8,
            9 => R9,
            10 => R10,
            11 => R11,
            12 => R12,
            13 => R13,
            14 => R14,
            15 => R15,
            e => {
                return Err(ParseError::InvalidMask {
                    invalid_set_bytes: { (e & !(0b1111)) as u32 },
                })
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CRegister {
    Cr0,
    Cr1,
    Cr2,
    Cr3,
    Cr4,
    Cr5,
    Cr6,
    Cr7,
    Cr8,
    Cr9,
    Cr10,
    Cr11,
    Cr12,
    Cr13,
    Cr14,
    Cr15,
}
impl TryFrom<u8> for CRegister {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, ParseError> {
        use CRegister::*;
        Ok(match value {
            0 => Cr0,
            1 => Cr1,
            2 => Cr2,
            3 => Cr3,
            4 => Cr4,
            5 => Cr5,
            6 => Cr6,
            7 => Cr7,
            8 => Cr8,
            9 => Cr9,
            10 => Cr10,
            11 => Cr11,
            12 => Cr12,
            13 => Cr13,
            14 => Cr14,
            15 => Cr15,
            e => {
                return Err(ParseError::InvalidMask {
                    invalid_set_bytes: { (e & !(0b1111)) as u32 },
                })
            }
        })
    }
}
pub const fn check_rest_null_mask(value: u32, mask: u32) -> Result<u32, ParseError> {
    if value & !mask > 0 {
        return Err(ParseError::InvalidMask {
            invalid_set_bytes: value & !mask,
        });
    } else {
        Ok(value & mask)
    }
}
pub const fn split_with_mask(value: u32, mask: u32) -> (u32, u32) {
    let first = value & mask;
    let second = value & !mask;
    (first, second)
}
pub mod consts {
    pub const COND_MASK: u32 = 0b1111 << 28;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelativeAdress(i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Adress(u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShifterOperand {
    Immediate(u32),
    Register(Register),
    ShiftLeft(u32),
}
impl TryFrom<u32> for ShifterOperand{
    type Error=ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let (i,rest)=split_with_mask(value, 1<<24);
        Ok(Self::Immediate(value))
    }
}
