use serde::{de, ser};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("eof")]
    Eof,

    #[error("expected integer")]
    ExpectedInteger,

    #[error("expected string")]
    ExpectedString,

    #[error("expected boolean")]
    ExpectedBoolean,
}

impl ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}
