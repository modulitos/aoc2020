use std::io::{BufRead, BufReader, Read};

use crate::convert_path_buf;
use crate::{Error, Result};
use regex::Regex;
use std::boxed::Box;
use std::path::PathBuf;

use crate::vec_ext::VecExt;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Bag(String); // represents the bags color

#[derive(Eq, PartialEq, Debug, Hash)]
struct Rule {
    container: Bag,
    items: Vec<(u8, Bag)>, // each tuple represents the number of bags that can fit inside the container.
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // eg: "dark orange bags contain 3 bright white bags, 4 muted yellow bags."
        //
        // eg: "bright white bags contain 1 shiny gold bag."
        lazy_static! {
            static ref CONTAINER_RE: Regex = Regex::new(
                r"(?x)
                (?P<color>\S+\s\S+)
                \s
                bags"
            )
            .unwrap();
            static ref ITEM_RE: Regex = Regex::new(
                r"(?x)
                (?P<count>\d+)
                \s
                (?P<color>\S+\s\S+)
                \s
                bag"
            )
            .unwrap();
        }

        let rule_parts = s.split("contain").collect::<Vec<&str>>();
        if rule_parts.len() != 2 {
            return Err(Error::InvalidInput(format!(
                "Invalid rule length: {}",
                rule_parts.len()
            )));
        }
        let container_input = rule_parts
            .get(0)
            .ok_or(Error::InvalidInput(format!(
                "Invalid container input: {:?}",
                rule_parts
            )))?
            .deref();
        let container_caps = CONTAINER_RE
            .captures(container_input)
            .ok_or(Error::InvalidInput(format!(
                "Invalid container capture on input: {}",
                container_input
            )))?;
        let container = Bag(container_caps["color"].parse()?);

        let items_input = rule_parts
            .get(1)
            .ok_or(Error::InvalidInput(format!("Invalid items input: {}", s)))?;
        let items = match items_input.contains("no other bags") {
            // When the bag has no match (eg: "faded blue bags contain no other bags.")
            true => Vec::new(),
            false => items_input
                .split(",")
                .map(|item_str| {
                    let caps = ITEM_RE
                        .captures(item_str)
                        .ok_or(Error::InvalidInput(format!(
                            "Invalid item input: {}",
                            item_str
                        )))?;
                    Ok((caps["count"].parse::<u8>()?, Bag(caps["color"].parse()?)))
                })
                .collect::<Result<Vec<(u8, Bag)>>>()?,
        };

        Ok(Self { container, items })
    }
}

// mapping of bags to their valid containers.
#[derive(Debug)]
struct BagsMap(HashMap<Bag, HashSet<Bag>>);

impl From<Vec<Rule>> for BagsMap {
    fn from(rules: Vec<Rule>) -> Self {
        let map = rules
            .into_iter()
            .fold(HashMap::<Bag, HashSet<Bag>>::new(), |mut map, rule| {
                let container = rule.container;
                rule.items.into_iter().for_each(|(_, item)| {
                    let containers = map.entry(item).or_default();
                    containers.insert(container.clone());
                });
                // make sure we have a mapping for our containers as well, even if it's empty:
                map.entry(container).or_default();

                map
            });
        Self(map)
    }
}

impl BagsMap {
    // Performs a BFS for all bags that contain the provided bag
    fn count_containing_bags(&self, bag: Bag) -> Result<u32> {
        let mut to_visit = HashSet::<&Bag>::from_iter(vec![&bag].into_iter());
        let mut visited = HashSet::<&Bag>::new();
        loop {
            if to_visit.is_empty() {
                visited.remove(&bag);
                return Ok(visited.len() as u32);
            }

            let to_visit_next = to_visit
                .iter()
                .map(|item_bag| {
                    self.0.get(item_bag).ok_or(Error::InvalidState(format!(
                        "Item bag is not found in our bag map! {:?}",
                        item_bag
                    )))
                })
                .collect::<Result<Vec<&HashSet<Bag>>>>()?
                .into_iter()
                .fold(HashSet::<&Bag>::new(), |mut set, container_bags| {
                    set.extend(container_bags.iter());
                    set
                })
                .difference(&visited)
                // TODO: avoid this double reference...
                .map(|bag| *bag)
                .collect::<HashSet<&Bag>>();

            let old_visited = std::mem::replace(&mut to_visit, to_visit_next);
            visited.extend(&old_visited);
        }
    }
}

pub fn part_1(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    let map = BagsMap::from(
        buf_reader
            .lines()
            .map(|line| line?.parse::<Rule>())
            .collect::<Result<Vec<Rule>>>()?,
    );
    map.count_containing_bags(Bag("shiny gold".into()))
}

pub fn part_2(mut buf_reader: BufReader<Box<dyn Read + '_>>) -> Result<u32> {
    Ok(32)
}

#[test]
fn test_parse_rule() -> Result<()> {
    let rule =
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".parse::<Rule>()?;
    assert_eq!(
        rule,
        Rule {
            container: Bag("dark orange".into()),
            items: vec![
                (3, Bag("bright white".into())),
                (4, Bag("muted yellow".into()))
            ]
        }
    );

    let rule = "bright white bags contain 1 shiny gold bag.".parse::<Rule>()?;
    assert_eq!(
        rule,
        Rule {
            container: Bag("bright white".into()),
            items: vec![(1, Bag("shiny gold".into()))]
        }
    );

    let rule = "faded blue bags contain no other bags.".parse::<Rule>()?;
    assert_eq!(
        rule,
        Rule {
            container: Bag("faded blue".into()),
            items: vec![]
        }
    );
    Ok(())
}

#[test]
fn test_parse_bags_map() -> Result<()> {
    let map = BagsMap::from(vec![
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".parse::<Rule>()?,
        "bright white bags contain 1 shiny gold bag.".parse::<Rule>()?,
        "faded blue bags contain no other bags.".parse::<Rule>()?,
    ]);

    assert_eq!(map.0.len(), 5);
    assert_eq!(
        map.0
            .get(&Bag("dark orange".into()))
            .map(|items| items.len()),
        Some(0)
    );
    assert_eq!(
        map.0.get(&Bag("shiny gold".into())),
        Some(&HashSet::from_iter(
            vec![Bag("bright white".into())].into_iter()
        ))
    );
    Ok(())
}

#[test]
fn test_part_1_example() -> Result<()> {
    let p = Some(PathBuf::from("./src/exercises/day_07/test.txt"));
    let res = part_1(convert_path_buf(p)?)?;

    assert_eq!(res, 4);
    Ok(())
}
