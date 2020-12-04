use std::io::{BufRead, BufReader, Read, Write, Error as IoError};

// TODO: dry this up:
// type Error = Box<dyn std::error::Error>;
use crate::{Error, Result };
// type Result<T, E = Error> = std::result::Result<T, E>;

pub fn part_1(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<()> {
    let receipts = buf_reader.lines().collect::<Result<Vec<String>, IoError>>()?;
    println!(
        "reading input lines: {:?}",

        // Note: we're passing in IoError explicitly. No sure why, but our crate's Error type does
        // not seem to be reading IoError, with message: "value of type
        // `std::result::Result<std::vec::Vec<std::string::String>, error::Error>` cannot be built
        // from `std::iter::Iterator<Item=std::result::Result<std::string::String,
        // std::io::Error>>`"

        receipts
    );

    println!("ok!");

    Ok(())
}
