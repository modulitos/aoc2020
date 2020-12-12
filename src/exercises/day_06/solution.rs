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
    any_yes_answers: HashSet<u8>,
    all_yes_answers: HashSet<u8>,
}

impl FromStr for Group {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let any_yes_answers = s.lines().fold(HashSet::new(), |mut answers, line| {
            answers.extend(line.as_bytes().into_iter().collect::<HashSet<&u8>>());
            answers
        });

        // // This is the Rust stable version, which doesn't use .fold_first:
        // let mut lines = s.lines();
        // let first = lines.nth(0).ok_or(Error::InvalidInput(format!(
        //     "Group must have at least one answer: {}",
        //     s
        // )))?.as_bytes().into_iter().collect::<HashSet<&u8>>();
        // let all_yes_answers = lines
        //     .fold(first, |answers, line| {
        //         // Take the intersection between each person's answers:
        //         &answers & &line.as_bytes().into_iter().collect::<HashSet<&u8>>()
        //     })
        //     .into_iter()
        //     .cloned()
        //     .collect::<HashSet<u8>>();

        let all_yes_answers = s
            .lines()
            .map(|line| line.as_bytes().into_iter().collect::<HashSet<&u8>>())
            // .fold_first only work on Rust Nightly!
            .fold_first(|answers, line| {
                // Take the intersection between each person's answers:
                &answers & &line
            })
            .ok_or(Error::InvalidState("unable to fold_first".into()))?.into_iter().cloned().collect();

        Ok(Self {
            any_yes_answers,
            all_yes_answers,
        })
    }
}

impl Group {
    fn get_any_yes_counts(&self) -> u32 {
        self.any_yes_answers.len() as u32
    }
    fn get_all_yes_counts(&self) -> u32 {
        self.all_yes_answers.len() as u32
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
        .map(|group| group.get_any_yes_counts())
        .sum::<u32>())
}
pub fn part_2(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    let mut input = String::new();
    buf_reader.read_to_string(&mut input);
    Ok(input
        .split("\n\n")
        .map(|group_str| group_str.parse::<Group>())
        .collect::<Result<Vec<Group>>>()?
        .into_iter()
        .map(|group| group.get_all_yes_counts())
        .sum::<u32>())
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
    assert_eq!(group.get_any_yes_counts(), 1);
    assert_eq!(group.get_all_yes_counts(), 1);

    let input = "\
        ab\n\
        ac\n\
    ";
    let group = input.parse::<Group>()?;
    assert_eq!(group.get_any_yes_counts(), 3);
    assert_eq!(group.get_all_yes_counts(), 1);
    Ok(())
}

#[test]
fn part_1_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_06/test.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 11);
    Ok(())
}

#[test]
fn test_part_1() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_06/answers.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 6565);
    Ok(())
}

#[test]
fn part_2_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_06/test.txt"));
    let res = part_2(convert_path_buf(p)?)?;

    assert_eq!(res, 6);
    Ok(())
}
#[test]
fn test_part_2() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_06/answers.txt"));
    let res = part_2(convert_path_buf(p)?)?;

    assert_eq!(res, 3137);
    Ok(())
}
