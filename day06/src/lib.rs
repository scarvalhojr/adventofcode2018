extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^(\d+),\s+(\d+)$").unwrap();

        let groups = pattern
            .captures(s)
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid format"))?;

        let numbers: Vec<i32> = groups
            .iter()
            .skip(1)
            .map(|val| {
                val.unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))
            }).collect::<Result<_, _>>()?;

        Ok(Point {
            x: numbers[0],
            y: numbers[1],
        })
    }
}

pub fn part1(points: &[Point]) -> i32 {
    // Forgive me Algorithm God for I have sinned...
    // this is a brute-force pile of ugliness...
    let mut infinite_areas = HashSet::new();
    let mut area_size: HashMap<&Point, i32> = HashMap::new();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    for x in min_x..=max_x {
        for y in &[min_y, max_y] {
            if let Some(point) = owner_point(x, *y, points) {
                infinite_areas.insert(point);
            }
        }
    }
    for y in min_y..=max_y {
        for x in &[min_x, max_x] {
            if let Some(point) = owner_point(*x, y, points) {
                infinite_areas.insert(point);
            }
        }
    }
    for x_pos in (min_x + 1)..max_x {
        for y_pos in (min_y + 1)..max_y {
            if let Some(point) = owner_point(x_pos, y_pos, points) {
                if !infinite_areas.contains(point) {
                    area_size
                        .entry(point)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
    }
    *area_size.values().max().unwrap_or(&0)
}

fn owner_point(x_pos: i32, y_pos: i32, points: &[Point]) -> Option<&Point> {
    let mut nearest = None;
    let mut min_dist = None;
    for point in points.iter() {
        let dist = manhattan_distance(x_pos, y_pos, point);
        if dist == 0 {
            return Some(point);
        }
        if min_dist.map_or(true, |min_dist| dist < min_dist) {
            nearest = Some(point);
            min_dist = Some(dist);
        } else if min_dist.map_or(false, |min_dist| dist == min_dist) {
            // Position may belong to multiple points
            nearest = None;
        }
    }
    nearest
}

fn manhattan_distance(x_pos: i32, y_pos: i32, point: &Point) -> i32 {
    (point.x - x_pos).abs() + (point.y - y_pos).abs()
}

pub fn part2(points: &[Point], _max_tot_dist: i32) -> i32 {
    // More brute-force uglinesses
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let mut count = 0;
    for x_pos in min_x..=max_x {
        for y_pos in min_y..=max_y {
            if points
                .iter()
                .map(|point| manhattan_distance(x_pos, y_pos, point))
                .sum::<i32>()
                < _max_tot_dist
            {
                count += 1
            }
        }
    }
    count
}
