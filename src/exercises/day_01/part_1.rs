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

    let mut start = 0;
    let mut end = receipts.len() - 1;
    loop {
        match (receipts.get(start), receipts.get(end)) {
            (Some(start_receipt), Some(end_receipt)) if start_receipt == end_receipt => {
                println!("none found!");
                return Ok(());
            }
            (Some(start_receipt), Some(end_receipt))
                if start_receipt.value + end_receipt.value > 2020 =>
            {
                end -= 1;
            }
            (Some(start_receipt), Some(end_receipt))
                if start_receipt.value + end_receipt.value < 2020 =>
            {
                start += 1;
            }
            (Some(start_receipt), Some(end_receipt))
                if start_receipt.value + end_receipt.value == 2020 =>
            {
                println!(
                    "receipt start: {:?}, receipt end: {:?}, product: {}",
                    start_receipt,
                    end_receipt,
                    start_receipt.value * end_receipt.value
                );
                return Ok(());
            }
            _ => {
                return Err(Error::InvalidState(format!(
                    "invalid state for start: {} and end: {}",
                    start, end
                )));
            }
        }
    }
}

pub fn part_2(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<()> {
    let receipts = get_receipts(buf_reader)?;
    Ok(())
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
struct Receipt {
    value: u32,
    id: usize,
}

impl TryFrom<(usize, String)> for Receipt {
    type Error = Error;
    fn try_from(src: (usize, String)) -> Result<Self, Self::Error> {
        let value = src.1.parse::<u32>()?;
        Ok(Self { id: src.0, value })
    }
}
