use crate::errors::DisasemblerError;
use crate::Options;

pub struct Parser {
    iter: Vec<u32>,
    over: Vec<u8>,
    using_little_endian:bool,
}
impl TryFrom<Options> for Parser {
    type Error = DisasemblerError;

    fn try_from(value: Options) -> Result<Self, Self::Error> {
        let file = std::fs::read(value.file).map_err(DisasemblerError::FileError)?;
        let (i, r) = file.as_chunks();
        let iter = i.into_iter().map(|e| u32::from_le_bytes(*e)).collect();
        Ok(Self {
            iter,
            over: r.to_vec(),
            using_little_endian:true,
        })
    }
}
