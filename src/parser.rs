use crate::errors::{DisasemblerError, ParseError};
use crate::instructions::arm::ArmInstruction;
use crate::Options;

pub struct Parser {
    pub iter: Vec<u32>,
    pub using_little_endian: bool,
}
impl Parser {
    pub fn parse(self) -> Result<Vec<ArmInstruction>, ParseError> {
        self.iter
            .into_iter()
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
impl TryFrom<Options> for Parser {
    type Error = DisasemblerError;

    fn try_from(value: Options) -> Result<Self, Self::Error> {
        let file = std::fs::read(value.file).map_err(DisasemblerError::FileError)?;
        let (i, r) = file.as_chunks();
        if r.len() > 0 {
            return Err(DisasemblerError::UnaligedFile(r.len()));
        }
        let iter = i.into_iter().map(|e| u32::from_le_bytes(*e)).collect();
        Ok(Self {
            iter,
            using_little_endian: true,
        })
    }
}
