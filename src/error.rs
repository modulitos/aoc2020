use std::io;
use std::num::TryFromIntError;
use std::io::Error as StdIoError;

use thiserror::Error;

// use io_error::IoError;

mod io_error;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    // #[error("IO Error")]
    // IoError(#[from] IoError),

    #[error("Std IO Error")]
    StdIoError(#[from] StdIoError)
}

// impl From<io::Error> for Error {
//     fn from(io_error: io::Error) -> Self {
//         Self::IoError(io_error.into())
//     }
// }