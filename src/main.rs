#![feature(slice_as_chunks)]
mod dsi;
pub mod errors;
mod instructions;
mod parser;
use std::path::PathBuf;
use clap::Parser as ClapParser;
use tracing::{debug, error};
use self::errors::DisasemblerError;
use self::parser::Parser;
#[derive(ClapParser)]
struct Options {
    #[clap( value_parser = file_exists )]
    file: PathBuf,
    #[clap(long, short, default_value = "false")]
    dsi: bool,
}
fn file_exists(v: &str) -> Result<PathBuf, String> {
    match std::fs::exists(v) {
        Ok(e) => match e {
            true => Ok(PathBuf::from(v)),
            false => Err("File does not exist".to_owned()),
        },
        Err(err) => {
            let string = format!("File could not be read {}", err);
            error!("{}", string);
            Err(string)
        }
    }
}
pub fn main() -> Result<(), DisasemblerError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let a: Parser = Options::parse().try_into()?;

    let v = a.parse()?;
    println!("{:?}", v);
    Ok(())
}
