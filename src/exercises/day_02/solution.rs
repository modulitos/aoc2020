use std::io::{BufRead, BufReader, Read};

use crate::convert_path_buf;
use crate::{Error, Result};
use regex::Regex;
use std::ops::Range;
use std::path::PathBuf;
use std::str::FromStr;

trait Policy {
    fn new(first: u8, second: u8, char: char) -> Self;
    fn is_valid(&self, pw: &Password) -> bool;
}

#[derive(Debug, Eq, PartialEq)]
struct Policy1 {
    range: Range<u8>,
    char: char,
}

impl Policy for Policy1 {
    fn new(first: u8, second: u8, char: char) -> Self {
        Self {
            range: std::ops::Range {
                start: first,
                end: second + 1,
            },
            char
        }
    }

    fn is_valid(&self, pw: &Password) -> bool {
        self.range
            .contains(&(pw.chars().filter(|char| char == &self.char).count() as u8))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PolicyWithPassword<P: Policy>((P, Password));

type Password = String;

impl<P: Policy> FromStr for PolicyWithPassword<P> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            // eg: `1-3 a: abcde`
            //
            // NOTE: the (?x) prefix allows us to escape white spaces
            static ref RE: Regex = Regex::new(
                r"(?x)
                    (?P<range_low>\d+)-(?P<range_high>\d+)
                    \s+
                    # char
                    (?P<char>[a-zA-Z]{1}):
                    \s+
                    # password
                    (?P<password>[a-zA-Z\d]+)
                    "
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        let policy = P::new(
            caps["range_low"].parse()?,
            caps["range_high"].parse::<u8>()?,
            caps["char"].parse()?
        );
        let password = caps["password"].parse()?;

        Ok(Self((policy, password)))
    }
}

fn get_policies<P: Policy>(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<Vec<PolicyWithPassword<P>>> {
    buf_reader
        .lines()
        .into_iter()
        .collect::<Result<Vec<String>, _>>()?
        .into_iter()
        .map(|line| PolicyWithPassword::from_str(&line))
        .collect()
}

pub fn part_1(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    let policies_with_passwords: Vec<PolicyWithPassword<Policy1>> = get_policies(buf_reader)?;

    Ok(policies_with_passwords
        .into_iter()
        .filter(|PolicyWithPassword((policy, password))| policy.is_valid(password))
        .count() as u32)
}
pub fn part_2(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    Ok(42)
}

#[test]
fn test_parser() -> Result<()> {
    let res = "1-3 a: abcde".parse::<PolicyWithPassword<Policy1>>()?;
    assert_eq!(
        res,
        PolicyWithPassword((
            Policy1 {
                range: Range { start: 1, end: 4 },
                char: 'a'
            },
            "abcde".into()
        ))
    );
    Ok(())
}

#[test]
fn test_part_1_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_02/test.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 2);
    Ok(())
}

#[test]
fn test_part_1() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_02/passwords.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 550);
    Ok(())
}
