use regex::Regex;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq)]
pub struct Point(i8, i8, i8, i8);

impl Point {
    fn is_near(&self, other: &Point) -> bool {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
            <= 3
    }
}

struct Constellation<'a> {
    points: HashSet<&'a Point>,
}

impl<'a> Constellation<'a> {
    fn new(point: &'a Point) -> Self {
        let mut points = HashSet::new();
        points.insert(point);
        Constellation { points }
    }

    fn is_near(&self, other: &Constellation) -> bool {
        self.points
            .iter()
            .any(|point| other.points.iter().any(|p| point.is_near(p)))
    }

    fn join(&mut self, mut other: Constellation<'a>) {
        self.points.extend(other.points.drain());
    }
}

pub fn part1(points: &[Point]) -> u32 {
    let mut count = 0;
    let mut pending: Vec<Constellation> =
        points.iter().map(Constellation::new).collect();
    while let Some(mut constellation) = pending.pop() {
        let near_idx = pending
            .iter()
            .enumerate()
            .find(|(_, other)| constellation.is_near(other))
            .map(|(idx, _)| idx);

        if let Some(idx) = near_idx {
            let near_const = pending.remove(idx);
            constellation.join(near_const);
            pending.push(constellation);
        } else {
            count += 1;
        }
    }

    count
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+),(-?\d+)$").unwrap();
        let groups = pattern.captures(s).ok_or("Invalid format")?;
        let numbers: Vec<i8> = groups
            .iter()
            .skip(1)
            .map(|val| {
                val.unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())
            })
            .collect::<Result<_, _>>()?;

        Ok(Point(numbers[0], numbers[1], numbers[2], numbers[3]))
    }
}
