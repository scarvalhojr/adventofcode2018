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
        let parts: Vec<&str> =
            s.trim_matches(|p| p == '#').split('@').collect();
        if parts.len() < 2 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid format"));
        }

        let id = parts[0]
            .trim()
            .parse::<u32>()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        let geometry: Vec<&str> = parts[1].split(':').collect();
        if geometry.len() < 2 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid format"));
        }

        let coords: Vec<&str> = geometry[0].split(',').collect();
        if coords.len() < 2 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid format"));
        }

        let coord_x = coords[0]
            .trim()
            .parse::<u32>()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        let coord_y = coords[1]
            .trim()
            .parse::<u32>()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        let dimension: Vec<&str> = geometry[1].split('x').collect();
        if dimension.len() < 2 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid format"));
        }

        let width = dimension[0]
            .trim()
            .parse::<u32>()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        let height = dimension[1]
            .trim()
            .parse::<u32>()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        Ok(Claim {
            id,
            coord_x,
            coord_y,
            width,
            height,
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
    if no_overlap.len() == 1 {
        *no_overlap.iter().next().unwrap()
    } else {
        0
    }
}
