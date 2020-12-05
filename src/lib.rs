#![warn(
clippy::all,
missing_debug_implementations,
rust_2018_idioms,
// missing_docs,
missing_doc_code_examples
)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]

use std::io::{BufReader, Read};
use exercises::day_01;
pub use args::Args;
pub use error::Error;

mod exercises;
mod args;

// type Error = Box<dyn std::error::Error>;
pub type Result<T, E=Error> = std::result::Result<T, E>;

mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn aoc(_day: usize, _part: usize, buf_reader: BufReader<Box<dyn Read>>) -> Result<()> {
    println!("starting aoc!");
    day_01::part_1(buf_reader)?;
    Ok(())
}
