#[derive(Debug, Copy, Clone)]
pub enum DictError {
    UnknownError,
}

pub type DictResult<T> = std::result::Result<T, DictError>;
