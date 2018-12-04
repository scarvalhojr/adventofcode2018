extern crate regex;

use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug)]
pub struct Claim {
    pub id: u32,
    pub coord_x: u32,
    pub coord_y: u32,
    pub width: u32,
    pub height: u32,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern =
            Regex::new(r"^#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();

        let groups = pattern
            .captures(s)
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid format"))?;

        let numbers: Vec<u32> = groups
            .iter()
            .skip(1)
            .map(|val| {
                val.unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))
            }).collect::<Result<_, _>>()?;

        Ok(Claim {
            id: numbers[0],
            coord_x: numbers[1],
            coord_y: numbers[2],
            width: numbers[3],
            height: numbers[4],
        })
    }
}

pub fn part1(claims: &[Claim]) -> u32 {
    let mut overlap: HashMap<(u32, u32), bool> = HashMap::new();
    claims.iter().for_each(|claim| {
        for pos_x in claim.coord_x..(claim.coord_x + claim.width) {
            for pos_y in claim.coord_y..(claim.coord_y + claim.height) {
                overlap
                    .entry((pos_x, pos_y))
                    .and_modify(|val| *val = true)
                    .or_insert(false);
            }
        }
    });
    overlap.values().filter(|&val| *val).count() as u32
}

pub fn part2(claims: &[Claim]) -> u32 {
    let mut first_claim: HashMap<(u32, u32), u32> = HashMap::new();
    let mut no_overlap: HashSet<u32> = HashSet::new();
    claims.iter().for_each(|claim| {
        no_overlap.insert(claim.id);
        for pos_x in claim.coord_x..(claim.coord_x + claim.width) {
            for pos_y in claim.coord_y..(claim.coord_y + claim.height) {
                match first_claim.entry((pos_x, pos_y)) {
                    Entry::Vacant(entry) => {
                        entry.insert(claim.id);
                    }
                    Entry::Occupied(entry) => {
                        no_overlap.remove(entry.get());
                        no_overlap.remove(&claim.id);
                    }
                }
            }
        }
    });
    *no_overlap.iter().next().unwrap_or(&0)
}
