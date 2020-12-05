#![warn(
clippy::all,
missing_debug_implementations,
rust_2018_idioms,
missing_doc_code_examples
)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]

pub use args::Args;
pub use error::Error;
use exercises::day_01;
use std::io::{BufReader, Read};
use aoc_result::AocReturn;
use option_ext::convert_path_buf;
use vec_ext::VecExt;

mod args;
mod exercises;
mod aoc_result;
mod option_ext;
mod vec_ext;

pub type Result<T, E = Error> = std::result::Result<T, E>;

mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn aoc(day: usize, part: usize, buf_reader: BufReader<Box<dyn Read>>) -> Result<AocReturn> {
    match (day, part) {
        (1, 1) => day_01::part_1(buf_reader).map(|v| v.into()),
        (1, 2) => day_01::part_2(buf_reader).map(|v| v.into()),
        _ => Err(Error::InvalidDayOrPartArg(day, part))
    }
}
