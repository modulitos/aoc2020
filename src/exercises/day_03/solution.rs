use std::io::{BufReader, Read};

use crate::convert_path_buf;
use crate::{Error, Result};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinate {
    y: usize,
    x: usize,
}

#[derive(Debug)]
enum Land {
    Open,
    Tree,
}

impl Land {
    fn from_byte(b: &u8) -> Result<Self> {
        match b {
            b'#' => Ok(Land::Tree),
            b'.' => Ok(Land::Open),
            b => Err(Error::InvalidState(format!(
                "Invalid input for Land: {}",
                b
            ))),
        }
    }
}

struct Area {
    width: usize,
    height: usize,
    map: BTreeMap<Coordinate, Land>,
}

impl FromStr for Area {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Traverse through the string to determine the extents of the Area:
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or("").len();
        if !s.lines().all(|line| line.len() == width) {
            return Err(Error::InvalidState(format!(
                "all lines in input must have the same length. Currently measured length: {}",
                width
            )));
        }

        let map = s
            .lines()
            .enumerate()
            // TODO: how to flat_map when the closure returns a Result<Iterator<_>> ?
            .map(|(y, line)| {
                line.as_bytes()
                    .into_iter()
                    .enumerate()
                    .map(|(x, b)| Ok((Coordinate { x, y }, Land::from_byte(b)?)))
                    .collect::<Result<Vec<(Coordinate, Land)>>>()
            })
            .collect::<Result<Vec<Vec<(Coordinate, Land)>>>>()?
            .into_iter()
            .fold(BTreeMap::new(), |mut map, row| {
                row.into_iter().for_each(|(coord, land)| {
                    map.insert(coord, land);
                });
                map
            });
        Ok(Self { width, height, map })
    }
}

struct Simulation {
    area: Area,
    user: Coordinate,
}

#[derive(Debug)]
struct Movement {
    dx: i32, // abs value must be less than width of Area.
    dy: i32, // abs value must be less than height of Area
}

impl Movement {
}

impl Simulation {
    fn create_movement(&self, dx: i32, dy: i32) -> Result<Movement> {
        let movement = Movement {
            dx,
            dy
        };

        // validate that the Movement makes sense in the context of the Area:
        if movement.dx + (self.area.width as i32) < 0 || movement.dy + (self.area.height as i32) < 0 {
            return Err(Error::InvalidState(format!(
                "Movement displacements exceed the Area's width: {}, height: {}, movement: {:?}",
                self.area.width, self.area.height, movement
            )));
        }

        Ok(movement)
    }

    // Returns the type of Land that the current user is on.
    //
    // If None, then the movement has taken the user off the map.
    //
    // User can only move off the map in the y direction. In the x direction, they just loop around.
    //
    fn take_move(&mut self, movement: Movement) -> Option<&Land> {
        self.user.x =
            (self.user.x + (movement.dx + (self.area.width as i32)) as usize) % self.area.width;
        self.user.y = ((self.user.y as i32) + (movement.dy)) as usize;

        self.area.map.get(&self.user)
    }

    // fn simulate_slope()
}

pub fn part_1(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;
    let area = input.parse::<Area>()?;
    let mut simulation = Simulation {
        area,
        user: Coordinate { x: 0, y: 0 },
    };
    let mut trees = 0;
    loop {
        // if simulation.user.
        match simulation.take_move(simulation.create_movement( 3, 1 )?) {
            Some(&Land::Tree) => trees += 1,
            Some(&Land::Open) => {}
            None => {
                // We're off the map - we've finished!
                return Ok(trees);
            }
        }
    }
}

pub fn part_2(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    Ok(42)
}

#[test]
fn test_part_1_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_03/test.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 7);
    Ok(())
}

#[test]
fn test_part_1() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_03/area.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 176);
    Ok(())
}
