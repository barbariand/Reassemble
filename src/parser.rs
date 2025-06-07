use crate::dsi::HeaderNDS;
use crate::errors::{DisasemblerError, ParseError};
use crate::instructions::arm::ArmInstruction;
use crate::Options;
use std::path::Path;
use tracing::{debug, warn};

pub struct Parser {
    pub iter_arm_9: Vec<u32>,
    rest: Vec<u8>,
    pub using_little_endian: bool,
    pub header: Option<HeaderNDS>,
}
impl Parser {
    pub fn parse(self) -> Result<Vec<ArmInstruction>, ParseError> {
        let offset = match self.header {
            Some(v) => v.arm9_offset,
            None => 0,
        } as usize;
        self.iter_arm_9
            .into_iter()
            .enumerate()
            .map(|(v, e)| {
                println!("{:#012x}    {:#032b}", v * 4 + offset, e);
                e
            })
            .map(|e| match e.try_into() {
                Ok(arm) => {
                    println!("{:?}", arm);
                    Ok(arm)
                }
                Err(e) => Err(e),
            })
            .collect()
    }
}
impl Parser {
    fn from_dsi(path: &Path) -> Result<Self, DisasemblerError> {
        let file = std::fs::read(path).map_err(DisasemblerError::FileError)?;
        if let Some((head, r)) = file.split_first_chunk::<0x1000>() {
            let header = HeaderNDS::from_bytes(*head);
            debug!("header is:{:#?}", header);
            debug!("arm9 offset is:{}", header.arm9_offset);
            debug!("Length of rest:{}", r.len());
            let iter = Self::from_bin(
                &r[header.arm9_offset as usize - 0x1000..][..header.arm9_size as usize],
            )?;
            Ok(Self {
                iter_arm_9: iter,
                rest: r.to_vec(),
                using_little_endian: true,
                header: Some(header),
            })
        } else {
            warn!("File was too short for the header, trying to dissasemble it as just binary");

            Ok(Self {
                iter_arm_9: Self::from_bin(&file)?,
                rest: vec![],
                using_little_endian: true,
                header: None,
            })
        }
    }
    fn from_bin(asm: &[u8]) -> Result<Vec<u32>, DisasemblerError> {
        let (i, r) = asm.as_chunks();
        if r.len() > 0 {
            return Err(DisasemblerError::UnaligedFile(r.len()));
        }
        Ok(i.into_iter().map(|e| u32::from_le_bytes(*e)).collect())
    }
    fn from_binary_file(path: &Path) -> Result<Self, DisasemblerError> {
        let file = std::fs::read(path).map_err(DisasemblerError::FileError)?;
        Ok(Self {
            iter_arm_9: Self::from_bin(&file)?,
            rest: vec![],
            using_little_endian: true,
            header: None,
        })
    }
}
impl TryFrom<Options> for Parser {
    type Error = DisasemblerError;

    fn try_from(value: Options) -> Result<Self, Self::Error> {
        match value.dsi {
            true => Self::from_dsi(&value.file),
            false => Self::from_binary_file(&value.file),
        }
    }
}
