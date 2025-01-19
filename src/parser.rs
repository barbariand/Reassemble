use std::mem::transmute;
use std::path::Path;

use crate::dsi::HeaderNDS;
use crate::errors::{DisasemblerError, ParseError};
use crate::instructions::arm::ArmInstruction;
use crate::Options;

pub struct Parser {
    pub iter: Vec<u32>,
    rest: Vec<u8>,
    pub using_little_endian: bool,
    pub header: HeaderNDS,
}
impl Parser {
    pub fn parse(self) -> Result<Vec<ArmInstruction>, ParseError> {
        self.iter
            .into_iter()
            .map(|e| {
                println!("{:2b}", e);
                e
            })
            .map(|e| match e.try_into() {
                Ok(v) => {
                    println!("{:?}", v);
                    Ok(v)
                }
                Err(e) => Err(e),
            })
            .collect()
    }
}
impl Parser {
    fn from_dsi(path: &Path) -> Result<Self, DisasemblerError> {
        let file = std::fs::read(path).map_err(DisasemblerError::FileError)?;
        let (head, r) = file
            .split_first_chunk::<4096>()
            .ok_or(DisasemblerError::FileToShort)?;
        let header: HeaderNDS = unsafe { transmute(*head) };

        let iter = Self::from_bin(
            &r[header.arm9_offset as usize - 0x1000..][..header.arm9_size as usize],
        )?;
        Ok(Self {
            iter,
            rest: r.to_vec(),
            using_little_endian: true,
            header,
        })
    }
    fn from_bin(asm: &[u8]) -> Result<Vec<u32>, DisasemblerError> {
        let (i, r) = asm.as_chunks();
        if r.len() > 0 {
            return Err(DisasemblerError::UnaligedFile(r.len()));
        }
        Ok(i.into_iter().map(|e| u32::from_le_bytes(*e)).collect())
    }
}
impl TryFrom<Options> for Parser {
    type Error = DisasemblerError;

    fn try_from(value: Options) -> Result<Self, Self::Error> {
        Self::from_dsi(&value.file)
    }
}
