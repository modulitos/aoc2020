use std::io::{BufReader, Read};

use crate::convert_path_buf;
use crate::{Error, Result};
use std::path::PathBuf;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;
use std::iter::FromIterator;


// Consider using a parser combinator instead of regexes here, like Nom: https://crates.io/crates/nom
lazy_static! {
            static ref BYR: Regex = Regex::new(
                r"(?x)
                    byr:(\d+)
                    "
            )
            .unwrap();
        }
lazy_static! {
            static ref IYR: Regex = Regex::new(
                r"(?x)
                     iyr:(\d+)
                    "
            )
            .unwrap();
        }
lazy_static! {
            static ref EYR: Regex = Regex::new(
                r"(?x)
                    eyr:(\d+)
                    "
            )
            .unwrap();
        }
lazy_static! {
            static ref HGT: Regex = Regex::new(
                r"(?x)
                    # hgt:([[:alpha:]\ \d]+)
                    hgt:(\S+)
                    "
            )
            .unwrap();
        }
lazy_static! {
            static ref HCL: Regex = Regex::new(
                r"(?x)
                    # hcl:([[:alpha:]a-zA-Z\#\d]+)
                    hcl:(\S+)
                    "
            )
            .unwrap();
        }
lazy_static! {
            static ref ECL: Regex = Regex::new(
                r"(?x)
                    # ecl:([[:alpha:]a-zA-Z\#\d]+)
                    ecl:(\S+)
                    "
            )
            .unwrap();
        }
lazy_static! {
            static ref PID: Regex = Regex::new(
                r"(?x)
                    pid:(\S+)
                    "
            )
            .unwrap();
        }
lazy_static! {
            static ref CID: Regex = Regex::new(
                r"(?x)
                    cid:(\S+)
                    "
            )
            .unwrap();
        }

#[derive(Debug, Hash, Eq, PartialEq)]
enum Field {
    BirthYear(u16),
    IssueYear(u16),
    ExpirationYear(u16),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(String)
}

#[derive(Debug, Eq, PartialEq)]
struct PassportInput {
    fields: HashSet<Field>
}

impl PassportInput {
    fn is_valid(&self) -> bool {
        match self.fields.len() {
            // All 8 fields are present:
            8 => true,
            // The only optional field is CountryId:
            7 => self.fields.iter().find(|field| {
                match field {
                    Field::CountryId(_) => true,
                    _ => false
                }
            }).is_none(),
            _ => false
        }
    }
}

impl FromStr for PassportInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = HashSet::new();
        use Field::*;

        // TODO: DRY this up
        if let Some(caps) = BYR.captures(s) {
            if BYR.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!("too many matches of regex BYR")));
            }
            fields.insert(BirthYear(caps[1].parse()?));
        }

        if let Some(caps) = IYR.captures(s) {
            if IYR.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!("too many matches of regex IYR")));
            }
            fields.insert(IssueYear(caps[1].parse()?));
        }

        if let Some(caps) = EYR.captures(s) {
            if EYR.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!("too many matches of regex EYR")));
            }
            fields.insert(ExpirationYear(caps[1].parse()?));
        }

        if let Some(caps) = HGT.captures(s) {
            if HGT.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!("too many matches of regex HGT")));
            }
            fields.insert(Height(caps[1].parse()?));
        }

        if let Some(caps) = HCL.captures(s) {
            if HCL.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!("too many matches of regex HCL")));
            }
            fields.insert(HairColor(caps[1].parse()?));
        }

        if let Some(caps) = ECL.captures(s) {
            if ECL.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!("too many matches of regex ECL")));
            }
            fields.insert(EyeColor(caps[1].parse()?));
        }

        if let Some(caps) = PID.captures(s) {
            if PID.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!("too many matches of regex PID")));
            }
            fields.insert(PassportId(caps[1].parse()?));
        }

        if let Some(caps) = CID.captures(s) {
            if CID.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!("too many matches of regex CID")));
            }
            fields.insert(CountryId(caps[1].parse()?));
        }
        Ok(Self {
            fields
        })
    }
}

fn get_passports_from_buffer(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<Vec<PassportInput>> {
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;
    input.split("\n\n").map(|passport_string| {
        passport_string.parse::<PassportInput>()
    }).collect::<Result<Vec<PassportInput>>>()
}

pub fn part_1(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<usize> {
    let passports = get_passports_from_buffer(buf_reader)?;
    println!("passports len: {:?}", passports.len());
    Ok(passports.iter().filter(|passport| passport.is_valid()).count())
}

pub fn part_2(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<usize> {
    Ok(42)

}

#[test]
fn test_from_string() -> Result<()> {
    use Field::*;
    let passport = "iyr:2013 ecl:amb cid:350 \
    \n eyr:2023 pid:028048884".parse::<PassportInput>()?;

    let fields = HashSet::from_iter(vec![
        IssueYear(2013),
        EyeColor("amb".into()),
        CountryId(350),
        ExpirationYear(2023),
        PassportId(028048884),
    ].into_iter());
    assert_eq!(passport, PassportInput {
        fields: fields
    });
    Ok(())
}

#[test]
fn test_from_string_err_duplicates() -> Result<()> {
    use Field::*;
    let passport = "iyr:2013 ecl:amb cid:350 eyr:2103 pid:028048884 eyr:2023".parse::<PassportInput>();
    assert_eq!(passport.is_err(), true);
    Ok(())
}

#[test]
fn test_get_passports_from_buffer() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_04/test.txt"));
    let passports = get_passports_from_buffer(convert_path_buf(p)?)?;
    assert_eq!(passports.len(), 4);
    Ok(())
}


#[test]
fn test_part_1_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_04/test.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 2);
    Ok(())
}

#[test]
fn test_part_1() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_04/passports.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 182);
    Ok(())
}

