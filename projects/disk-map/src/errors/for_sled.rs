use sled::Error;

use crate::{DictError, DictError::CustomError};

impl From<Error> for DictError {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(e) => Self::IOError(e),
            _ => CustomError(error.to_string()),
        }
    }
}
