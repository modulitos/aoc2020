use std::io::{BufRead, BufReader, Error as IoError, Read};

use crate::convert_path_buf;
use crate::option_ext::OptionExt;
use crate::vec_ext::VecExt;
use crate::{Error, Result};
use std::convert::TryInto;
use std::path::PathBuf;

fn get_receipts(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<Vec<Receipt>> {
    Ok(buf_reader
        .lines()
        .collect::<Result<Vec<String>, _>>()? // elided io::IoError
        .into_iter()
        .map(|value| value.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()? // elided num::ParseIntError
        .sorted()
        .into_iter()
        .enumerate()
        .map(Receipt::from)
        .collect())
}

fn search_combinations(receipts: &Vec<Receipt>, third: Option<&Receipt>) -> Result<Option<u32>> {
    let mut end = receipts.len() - 1;
    let (mut start, extra_receipt_value) = match third {
        Some(receipt) => (std::cmp::min(receipt.id + 1, end), receipt.value),
        None => (0, 0),
    };
    loop {
        match (receipts.get(start), receipts.get(end)) {
            (Some(start_receipt), Some(end_receipt)) if start_receipt == end_receipt => {
                return Ok(None);
            }
            (Some(start_receipt), Some(end_receipt))
                if start_receipt.value + end_receipt.value + extra_receipt_value > 2020 =>
            {
                end -= 1;
            }
            (Some(start_receipt), Some(end_receipt))
                if start_receipt.value + end_receipt.value + extra_receipt_value < 2020 =>
            {
                start += 1;
            }
            (Some(start_receipt), Some(end_receipt))
                if start_receipt.value + end_receipt.value + extra_receipt_value == 2020 =>
            {
                return Ok(Some(start_receipt.value * end_receipt.value));
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

pub fn part_1(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<Option<u32>> {
    let receipts = get_receipts(buf_reader)?;
    search_combinations(&receipts, None)
}

pub fn part_2(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<Option<u32>> {
    let receipts = get_receipts(buf_reader)?;
    let res = receipts.iter().find_map(|receipt| {
        // println!("iterating over receipt: {:?}", receipt);
        match search_combinations(&receipts, Some(receipt)) {
            Ok(Some(product)) => Some(product * receipt.value),
            Ok(None) => None,

            // TODO: We can avoid this panic, and keep a declarative style, by using `try_find`,
            // only available on nightly:
            // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_find
            Err(err) => panic!("error: {:?}", err),
        }
    });
    Ok(res)
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
struct Receipt {
    value: u32,
    id: usize,
}

impl From<(usize, u32)> for Receipt {
    fn from(src: (usize, u32)) -> Self {
        Self {
            id: src.0,
            value: src.1,
        }
    }
}

#[test]
fn test_part_1_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_01/test.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, Some(514579));
    Ok(())
}

#[test]
fn test_part_1() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_01/receipts.txt"));
    let res = part_1(convert_path_buf(p)?)?;
    assert_eq!(res, Some(876459));

    Ok(())
}

#[test]
fn test_part_2_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_01/test.txt"));
    let res = part_2(convert_path_buf(p)?)?;

    assert_eq!(res, Some(241861950));
    Ok(())
}

#[test]
fn test_part_2() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_01/receipts.txt"));
    let res = part_2(convert_path_buf(p)?)?;

    assert_eq!(res, Some(116168640));
    Ok(())
}
