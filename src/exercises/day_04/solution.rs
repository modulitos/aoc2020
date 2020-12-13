use std::io::{BufReader, Read};

use crate::convert_path_buf;
use crate::{Error, Result};
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::str::FromStr;

// Consider using a parser combinator instead of regexes here, like Nom: https://crates.io/crates/nom
lazy_static! {
    static ref BYR: Regex = Regex::new(r"byr:(\d+)").expect("ok");
    static ref IYR: Regex = Regex::new(r"iyr:(\d+)").expect("ok");
    static ref EYR: Regex = Regex::new(r"eyr:(\d+)").unwrap();
    static ref HGT: Regex = Regex::new(r"hgt:(\S+)").unwrap();
    static ref HCL: Regex = Regex::new(r"hcl:(\S+)").unwrap();
    static ref ECL: Regex = Regex::new(r"ecl:(\S+)").unwrap();
    static ref PID: Regex = Regex::new(r"pid:(\S+)").unwrap();
    static ref CID: Regex = Regex::new(r"cid:(\S+)").unwrap();
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
    CountryId(String),
}

// These are the regexes we're using for validation:
lazy_static! {
    static ref HEIGHT_PARSER: Regex = Regex::new(r"^(?P<value>\d+)(?P<unit>in|cm)$").unwrap();
    static ref HAIR_COLOR_PARSER: Regex = Regex::new(r"^\#([0-9a-f]{6})$").unwrap();
    static ref EYE_COLOR_PARSER: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PASSPORT_ID_PARSER: Regex = Regex::new(r"^(\d{9})$").unwrap();
}
impl Field {
    fn is_valid_part_2(&self) -> bool {
        use Field::*;
        // TODO: we can model this better with a "ValidPassportInput" struct...
        match self {
            BirthYear(1920..=2002) => true,
            IssueYear(2010..=2020) => true,
            ExpirationYear(2020..=2030) => true,
            Height(field) => HEIGHT_PARSER.captures(&field).map_or(false, |caps| {
                match (caps["value"].parse::<u16>(), &caps["unit"]) {
                    (Ok(value), "cm") => 150 <= value && value <= 193,
                    (Ok(value), "in") => 59 <= value && value <= 76,
                    _ => false,
                }
            }),
            HairColor(field) => HAIR_COLOR_PARSER
                .captures(&field)
                .map_or(false, |caps| caps[0].parse::<String>().is_ok()),
            EyeColor(field) => EYE_COLOR_PARSER
                .captures(&field)
                .map_or(false, |caps| caps[0].parse::<String>().is_ok()),
            PassportId(field) => PASSPORT_ID_PARSER
                .captures(&field)
                .map_or(false, |caps| caps[0].parse::<String>().is_ok()),
            CountryId(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PassportInput {
    fields: HashSet<Field>,
}

impl PassportInput {
    fn is_valid(&self) -> bool {
        match self.fields.len() {
            // All 8 fields are present:
            8 => true,
            // The only optional field is CountryId:
            7 => self
                .fields
                .iter()
                .find(|field| match field {
                    Field::CountryId(_) => true,
                    _ => false,
                })
                .is_none(),
            _ => false,
        }
    }

    fn is_valid_part_2(&self) -> bool {
        self.is_valid() && self.fields.iter().all(|field| field.is_valid_part_2())
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
                return Err(Error::InvalidState(format!(
                    "too many matches of regex BYR"
                )));
            }
            fields.insert(BirthYear(caps[1].parse()?));
        }

        if let Some(caps) = IYR.captures(s) {
            if IYR.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!(
                    "too many matches of regex IYR"
                )));
            }
            fields.insert(IssueYear(caps[1].parse()?));
        }

        if let Some(caps) = EYR.captures(s) {
            if EYR.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!(
                    "too many matches of regex EYR"
                )));
            }
            fields.insert(ExpirationYear(caps[1].parse()?));
        }

        if let Some(caps) = HGT.captures(s) {
            if HGT.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!(
                    "too many matches of regex HGT"
                )));
            }
            fields.insert(Height(caps[1].parse()?));
        }

        if let Some(caps) = HCL.captures(s) {
            if HCL.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!(
                    "too many matches of regex HCL"
                )));
            }
            fields.insert(HairColor(caps[1].parse()?));
        }

        if let Some(caps) = ECL.captures(s) {
            if ECL.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!(
                    "too many matches of regex ECL"
                )));
            }
            fields.insert(EyeColor(caps[1].parse()?));
        }

        if let Some(caps) = PID.captures(s) {
            if PID.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!(
                    "too many matches of regex PID"
                )));
            }
            fields.insert(PassportId(caps[1].parse()?));
        }

        if let Some(caps) = CID.captures(s) {
            if CID.find_iter(s).count() > 1 {
                return Err(Error::InvalidState(format!(
                    "too many matches of regex CID"
                )));
            }
            fields.insert(CountryId(caps[1].parse()?));
        }
        Ok(Self { fields })
    }
}

fn get_passports_from_buffer(
    mut buf_reader: BufReader<Box<dyn Read + '_>>,
) -> Result<Vec<PassportInput>> {
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;
    input
        .split("\n\n")
        .map(|passport_string| passport_string.parse::<PassportInput>())
        .collect::<Result<Vec<PassportInput>>>()
}

pub fn part_1(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<usize> {
    let passports = get_passports_from_buffer(buf_reader)?;
    Ok(passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count())
}

pub fn part_2(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<usize> {
    let passports = get_passports_from_buffer(buf_reader)?;
    Ok(passports
        .iter()
        .filter(|passport| passport.is_valid_part_2())
        .count())
}

#[test]
fn test_from_string() -> Result<()> {
    use Field::*;
    let passport = "iyr:2013 ecl:amb cid:350 \
    \n eyr:2023 pid:028048884"
        .parse::<PassportInput>()?;

    let fields = HashSet::from_iter(
        vec![
            IssueYear(2013),
            EyeColor("amb".into()),
            CountryId("350".into()),
            ExpirationYear(2023),
            PassportId("028048884".into()),
        ]
        .into_iter(),
    );
    assert_eq!(passport, PassportInput { fields: fields });
    Ok(())
}

#[test]
fn test_from_string_err_duplicates() -> Result<()> {
    use Field::*;
    let passport =
        "iyr:2013 ecl:amb cid:350 eyr:2103 pid:028048884 eyr:2023".parse::<PassportInput>();
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
fn test_height_field_validator() -> Result<()> {
    let field1 = Field::Height("150cm".into());
    assert_eq!(field1.is_valid_part_2(), true);

    let invalid_field = Field::Height("149cm".into());
    assert_eq!(invalid_field.is_valid_part_2(), false);

    let field2 = Field::Height("76in".into());
    assert_eq!(field2.is_valid_part_2(), true);

    let field3 = Field::Height("x76in".into());
    assert_eq!(field3.is_valid_part_2(), false);
    Ok(())
}

#[test]
fn test_hair_field_validator() -> Result<()> {
    let field1 = Field::HairColor("#60292f".into());
    assert_eq!(field1.is_valid_part_2(), true);

    let invalid_field = Field::HairColor("1f7352".into());
    assert_eq!(invalid_field.is_valid_part_2(), false);

    let field2 = Field::HairColor("#60292z".into()); // not a-f
    assert_eq!(field2.is_valid_part_2(), false);

    let field3 = Field::HairColor("#60292f0".into()); // 7 digits
    assert_eq!(field3.is_valid_part_2(), false);

    Ok(())
}

#[test]
fn test_eye_color_field_validator() -> Result<()> {
    let field = Field::EyeColor("amb".into());
    assert_eq!(field.is_valid_part_2(), true);

    let field = Field::EyeColor("blu".into());
    assert_eq!(field.is_valid_part_2(), true);

    let field = Field::EyeColor("oth".into());
    assert_eq!(field.is_valid_part_2(), true);

    let field = Field::EyeColor("amb ".into());
    assert_eq!(field.is_valid_part_2(), false);

    let field = Field::EyeColor("ambx".into());
    assert_eq!(field.is_valid_part_2(), false);

    let field = Field::EyeColor("amb blu".into());
    assert_eq!(field.is_valid_part_2(), false);

    Ok(())
}

#[test]
fn test_passport_id_field_validator() -> Result<()> {
    let field = Field::PassportId("157096267".into());
    assert_eq!(field.is_valid_part_2(), true);

    let field = Field::PassportId("000096267".into());
    assert_eq!(field.is_valid_part_2(), true);

    let field = Field::PassportId(" 000096267".into());
    assert_eq!(field.is_valid_part_2(), false);

    let field = Field::PassportId("00096267".into());
    assert_eq!(field.is_valid_part_2(), false);

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

#[test]
fn test_part_2_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_04/test_2.txt"));
    let res = part_2(convert_path_buf(p)?)?;

    assert_eq!(res, 4);
    Ok(())
}

#[test]
fn test_part_2() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_04/passports.txt"));
    let res = part_2(convert_path_buf(p)?)?;

    assert_eq!(res, 109);
    Ok(())
}
