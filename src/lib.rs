#![warn(
    clippy::all,
    missing_debug_implementations,
    rust_2018_idioms,
    missing_doc_code_examples
)]
#![feature(iterator_fold_self)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;

use aoc_result::AocReturn;
pub use args::Args;
pub use error::Error;
use exercises::{day_01, day_02, day_03, day_04, day_05, day_06, day_07};
use option_ext::convert_path_buf;
use std::io::{BufReader, Read};
use vec_ext::VecExt;

mod aoc_result;
mod args;
mod exercises;
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
        (2, 1) => day_02::part_1(buf_reader).map(|v| v.into()),
        (2, 2) => day_02::part_2(buf_reader).map(|v| v.into()),
        (3, 1) => day_03::part_1(buf_reader).map(|v| v.into()),
        (3, 2) => day_03::part_2(buf_reader).map(|v| v.into()),
        (4, 1) => day_04::part_1(buf_reader).map(|v| v.into()),
        (4, 2) => day_04::part_2(buf_reader).map(|v| v.into()),
        (5, 1) => day_05::part_1(buf_reader).map(|v| v.into()),
        (5, 2) => day_05::part_2(buf_reader).map(|v| v.into()),
        (6, 1) => day_06::part_1(buf_reader).map(|v| v.into()),
        (6, 2) => day_06::part_2(buf_reader).map(|v| v.into()),
        (7, 1) => day_07::part_1(buf_reader).map(|v| v.into()),
        (7, 2) => day_07::part_2(buf_reader).map(|v| v.into()),
        _ => Err(Error::InvalidDayOrPartArg(day, part)),
    }
}
