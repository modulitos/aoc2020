use std::io::Error as StdIoError;
use std::num::ParseIntError as StdParseIntError;

use thiserror::Error;

use io_error::IoError;
use parse_int_error::ParseIntError;

mod io_error;
mod parse_int_error;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error")]
    IoError(#[from] IoError),

    #[error("Parse IntError")]
    ParseIntError(#[from] ParseIntError),
}

impl From<StdIoError> for Error {
    fn from(io_error: StdIoError) -> Self {
        Self::IoError(io_error.into())
    }
}

impl From<StdParseIntError> for Error {
    fn from(pi_error: StdParseIntError) -> Self {
        Self::ParseIntError(pi_error.into())
    }
}
