use serde_binary::Error;

use crate::{DictError, DictError::CustomError};

impl From<Error> for DictError {
    fn from(error: Error) -> Self {
        CustomError(error.to_string())
    }
}
