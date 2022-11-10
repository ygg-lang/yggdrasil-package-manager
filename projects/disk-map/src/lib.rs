#[doc = include_str!("../Readme.md")]
#[forbid(missing_docs)]
pub use database::DiskMap;
pub use errors::{DictError, Result};

mod database;
mod errors;
