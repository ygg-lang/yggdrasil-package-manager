use std::{
    error::Error,
    fmt::{Display, Formatter},
};

mod for_binary;
mod for_sled;
mod for_std;

#[derive(Debug)]
pub enum DictError {
    IOError(std::io::Error),
    CustomError(String),
    KeyNotFound,
}

pub type Result<T = ()> = std::result::Result<T, DictError>;

impl Display for DictError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DictError::IOError(e) => {
                write!(f, "IO Error: {e}")
            }
            DictError::CustomError(e) => f.write_str(e),
            DictError::KeyNotFound => write!(f, "No such key"),
        }
    }
}

impl Error for DictError {}
