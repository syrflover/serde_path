mod error;
mod ser;

pub use error::Error;
pub use ser::*;

pub type Result<T> = std::result::Result<T, Error>;
