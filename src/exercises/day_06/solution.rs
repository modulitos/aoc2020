use std::io::{BufRead, BufReader, Read};

use crate::convert_path_buf;
use crate::{Error, Result};
use std::boxed::Box;
use std::path::PathBuf;

use crate::vec_ext::VecExt;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::str::FromStr;

struct Group {
    yes_answers: HashSet<u8>,
}

impl FromStr for Group {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let yes_answers = s.lines().fold(HashSet::new(), |mut answers, line| {
            answers.extend(line.as_bytes().into_iter().collect::<HashSet<&u8>>());
            answers
        });
        Ok(Self { yes_answers })
    }
}

impl Group {
    fn get_counts(&self) -> u32 {
        self.yes_answers.len() as u32
    }
}

pub fn part_1(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    let mut input = String::new();
    buf_reader.read_to_string(&mut input);
    Ok(input
        .split("\n\n")
        .map(|group_str| group_str.parse::<Group>())
        .collect::<Result<Vec<Group>>>()?
        .into_iter()
        .map(|group| group.get_counts())
        .sum::<u32>())
}
pub fn part_2(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    Ok(32)
}

#[test]
fn make_group() -> Result<()> {
    let input = "\
        a\n\
        a\n\
        a\n\
        a\n\
    ";
    let group = input.parse::<Group>()?;
    assert_eq!(group.get_counts(), 1);

    let input = "\
        ab\n\
        ac\n\
    ";
    let group = input.parse::<Group>()?;
    assert_eq!(group.get_counts(), 3);
    Ok(())
}

#[test]
fn part_1_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_06/test.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 11);
    Ok(())
}
