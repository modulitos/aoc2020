use std::io::{BufRead, BufReader, Error as IoError, Read};

use crate::{Error, Result};
use itertools::Itertools;
use std::convert::TryFrom;

fn get_receipts(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<Vec<Receipt>> {
    Ok(buf_reader
        .lines()
        .collect::<Result<Vec<String>, _>>()? // elided io::IoError
        .into_iter()
        .enumerate()
        .map(Receipt::try_from)
        .collect::<Result<Vec<Receipt>, _>>()? // elided num::ParseIntError
        .into_iter()
        .sorted()
        .collect())
}

pub fn part_1(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<()> {
    let receipts = get_receipts(buf_reader)?;

    println!("reading input lines: {:?}", receipts);

    println!("ok!");

    Ok(())
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
struct Receipt {
    id: usize,
    value: u32,
}

impl TryFrom<(usize, String)> for Receipt {
    type Error = Error;
    fn try_from(src: (usize, String)) -> Result<Self, Self::Error> {
        let value = src.1.parse::<u32>()?;
        Ok(Self { id: src.0, value })
    }
}
