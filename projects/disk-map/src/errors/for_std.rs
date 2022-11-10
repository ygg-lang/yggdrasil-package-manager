use crate::DictError;

impl From<std::io::Error> for DictError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
