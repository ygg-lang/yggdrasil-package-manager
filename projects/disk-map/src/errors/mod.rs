use std::{
    error::Error,
    fmt::{Display, Formatter},
};

mod for_sled;

#[derive(Debug, Clone)]
pub enum DictError {
    IOError(std::io::Error),
    CustomError(String),
    KeyNotFound(Vec<u8>),
}

pub type DictResult<T = ()> = Result<T, DictError>;

impl Display for DictError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for DictError {}
