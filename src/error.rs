use serde::ser;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
