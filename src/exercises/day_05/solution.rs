use std::io::{BufRead, BufReader, Read};

use crate::convert_path_buf;
use crate::{Error, Result};
use std::boxed::Box;
use std::path::PathBuf;

use std::convert::TryFrom;
use std::str::FromStr;

trait BinaryEnum: TryFrom<u8, Error = Error> {
    // Returns either 0 or 1
    fn get_bit(&self) -> u8;
}

enum RowDirection {
    Front,
    Back,
}

impl BinaryEnum for RowDirection {
    fn get_bit(&self) -> u8 {
        use RowDirection::*;
        match self {
            Front => 0,
            Back => 1,
        }
    }
}

impl TryFrom<u8> for RowDirection {
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'F' => Ok(Self::Front),
            b'B' => Ok(Self::Back),
            _ => Err(Error::InvalidInput(format!(
                "invalid byte encountered for RowDirection: {}",
                b
            ))),
        }
    }
}

enum SeatDirection {
    Left,
    Right,
}
impl BinaryEnum for SeatDirection {
    fn get_bit(&self) -> u8 {
        use SeatDirection::*;
        match self {
            Left => 0,
            Right => 1,
        }
    }
}

impl TryFrom<u8> for SeatDirection {
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'L' => Ok(Self::Left),
            b'R' => Ok(Self::Right),
            _ => Err(Error::InvalidInput(format!(
                "invalid byte encountered for SeatDirection: {}",
                b
            ))),
        }
    }
}

struct Instructions<T: BinaryEnum>(Vec<T>);

impl<T: BinaryEnum> FromStr for Instructions<T> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.as_bytes().iter().cloned().collect::<Vec<u8>>();
        bytes.reverse(); // reversing because we want the LSB at the end.
        let enums = bytes
            .into_iter()
            .map(|byte| T::try_from(byte))
            .collect::<Result<Vec<T>>>()?;
        Ok(Instructions(enums))
    }
}
impl<T: BinaryEnum> Instructions<T> {
    fn get_value(&self) -> u32 {
        let base: u32 = 2;
        self.0
            .iter()
            .enumerate()
            .map(|(i, binary_enum)| u32::from(binary_enum.get_bit()) * base.pow(i as u32))
            .sum()
    }
}

struct SeatAssignment {
    row_instructions: Instructions<RowDirection>,
    seat_instructions: Instructions<SeatDirection>,
}

impl FromStr for SeatAssignment {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(Error::InvalidInput(format!(
                "SeatAssignment must be created from a str of length 10, not {}",
                s.len()
            )));
        }
        let (rows, seats) = s.split_at(7);
        Ok(Self {
            row_instructions: rows.parse::<Instructions<RowDirection>>()?,
            seat_instructions: seats.parse::<Instructions<SeatDirection>>()?,
        })
    }
}

impl SeatAssignment {
    fn get_seat_id(&self) -> u32 {
        self.row_instructions.get_value() * 8 + self.seat_instructions.get_value()
    }
}

pub fn part_1(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    buf_reader
        .lines()
        .into_iter()
        .map(|line| Ok(line?.parse::<SeatAssignment>()?.get_seat_id()))
        .collect::<Result<Vec<u32>>>()?
        .into_iter()
        .max()
        .ok_or(Error::InvalidState(
            "no valid seat id's can be derived from input".into(),
        ))
}

pub fn part_2(buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    Ok(42)
}

#[test]
fn test_bool() -> Result<()> {
    assert_eq!(u8::from(true), 1);
    Ok(())
}

#[test]
fn parse_row_direction() -> Result<()> {
    let instructions = "FBBBF".parse::<Instructions<RowDirection>>()?;
    assert_eq!(
        instructions
            .0
            .into_iter()
            .map(|instruction| instruction.get_bit())
            .sum::<u8>(),
        3
    );
    Ok(())
}

#[test]
fn parse_seat_direction() -> Result<()> {
    let instructions = "RLR".parse::<Instructions<SeatDirection>>()?;
    assert_eq!(
        instructions
            .0
            .into_iter()
            .map(|instruction| instruction.get_bit())
            .sum::<u8>(),
        2
    );
    Ok(())
}

#[test]
fn value_of_instructions() -> Result<()> {
    let instructions = "RLR".parse::<Instructions<SeatDirection>>()?;
    assert_eq!(instructions.get_value(), 5);

    let instructions = "FBBBF".parse::<Instructions<RowDirection>>()?;
    assert_eq!(instructions.get_value(), 14);
    Ok(())
}

#[test]
fn value_of_seat_assignment() -> Result<()> {
    let assignment = "FBFBBFFRLR".parse::<SeatAssignment>()?;
    assert_eq!(assignment.get_seat_id(), 357);

    let assignment = "BFFFBBFRRR".parse::<SeatAssignment>()?;
    assert_eq!(assignment.get_seat_id(), 567);

    let assignment = "FFFBBBFRRR".parse::<SeatAssignment>()?;
    assert_eq!(assignment.get_seat_id(), 119);
    Ok(())
}

#[test]
fn test_part_1_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_05/test.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 820);
    Ok(())
}

#[test]
fn test_part_1() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_05/seat_assignments.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 892);
    Ok(())
}
