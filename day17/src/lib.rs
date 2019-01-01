use crate::Area::*;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

type Coordinate = i32;

// Position is (Y, X)
type Position = (Coordinate, Coordinate);

const SPRING_POS: Position = (0, 500);

#[derive(PartialEq)]
enum Area {
    Spring,
    Clay,
    Sand,
    MovingWater,
    RetainedWater,
}

impl Area {
    fn is_unblocked(&self) -> bool {
        match self {
            Sand => true,
            MovingWater => true,
            _ => false,
        }
    }

    fn is_wet(&self) -> bool {
        match self {
            MovingWater => true,
            RetainedWater => true,
            _ => false,
        }
    }
}

pub struct Reservoir {
    area: HashMap<Position, Area>,
    spring_pos: Position,
    top: Coordinate,
    bottom: Coordinate,
}

impl Reservoir {
    pub fn fill(&mut self) {
        let pos_below = |pos: &Position| (pos.0 + 1, pos.1);

        let mut queue = VecDeque::new();
        queue.push_back(pos_below(&self.spring_pos));

        while let Some(mut pos) = queue.pop_front() {
            // Water falls until it hits something
            while self.get_area(&pos_below(&pos)).is_unblocked()
                && pos.0 <= self.bottom
            {
                self.area.insert(pos, MovingWater);
                pos = pos_below(&pos);
            }

            // Done if bottom of reservoir was reached
            if pos.0 >= self.bottom {
                continue;
            }

            loop {
                // Spread to the left
                let mut left = pos;
                let mut left_wall = true;
                while self.get_area(&left).is_unblocked() {
                    self.area.insert(left, MovingWater);
                    if self.get_area(&pos_below(&left)).is_unblocked() {
                        // no wall on the left
                        left_wall = false;
                        if !queue.contains(&left) {
                            queue.push_back(left);
                        }
                        break;
                    }
                    left.1 -= 1;
                }

                // Spread to the right
                let mut right = pos;
                let mut right_wall = true;
                while self.get_area(&right).is_unblocked() {
                    self.area.insert(right, MovingWater);
                    if self.get_area(&pos_below(&right)).is_unblocked() {
                        // no wall on the right
                        right_wall = false;
                        if !queue.contains(&right) {
                            queue.push_back(right);
                        }
                        break;
                    }
                    right.1 += 1;
                }

                if left_wall && right_wall {
                    // If there are walls on both sides, fill it with water...
                    for x in left.1 + 1..right.1 {
                        self.area.insert((pos.0, x), RetainedWater);
                    }
                    // ...and move up
                    pos.0 -= 1;
                } else {
                    break;
                }
            }
        }
    }

    pub fn total_wet(&self) -> usize {
        self.area
            .iter()
            .filter(|(pos, area)| pos.0 >= self.top && area.is_wet())
            .count()
    }

    pub fn total_retained_water(&self) -> usize {
        self.area
            .values()
            .filter(|&area| *area == RetainedWater)
            .count()
    }

    fn get_area(&self, position: &Position) -> &Area {
        self.area.get(position).unwrap_or(&Sand)
    }
}

impl FromStr for Reservoir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut area = HashMap::new();
        let pattern = Regex::new(r"^([xy])=(\d+), [xy]=(\d+)..(\d+)$").unwrap();

        for wall in s.lines() {
            let captures = pattern.captures(wall).ok_or("Invalid format")?;
            let mut tokens = captures.iter().skip(1);
            let orientation = tokens.next().unwrap().unwrap().as_str();
            let numbers: Vec<Coordinate> = tokens
                .map(|val| {
                    val.unwrap()
                        .as_str()
                        .parse()
                        .map_err(|err: ParseIntError| err.to_string())
                })
                .collect::<Result<_, _>>()?;
            match orientation {
                "x" => {
                    for pos_y in numbers[1]..=numbers[2] {
                        area.insert((pos_y, numbers[0]), Clay);
                    }
                }
                _ => {
                    for pos_x in numbers[1]..=numbers[2] {
                        area.insert((numbers[0], pos_x), Clay);
                    }
                }
            }
        }

        let top = area.keys().map(|a| a.0).min().ok_or("empty reservoir")?;
        let bottom = area.keys().map(|a| a.0).max().unwrap();
        area.insert(SPRING_POS, Spring);

        Ok(Self {
            area,
            spring_pos: SPRING_POS,
            top,
            bottom,
        })
    }
}

impl Display for Reservoir {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let min_x = self.area.keys().map(|a| a.1).min().unwrap();
        let max_x = self.area.keys().map(|a| a.1).max().unwrap();

        let display = (self.spring_pos.0..=self.bottom)
            .map(|pos_y| {
                (min_x..=max_x)
                    .map(|pos_x| match self.get_area(&(pos_y, pos_x)) {
                        Spring => '+',
                        Sand => '.',
                        Clay => '#',
                        MovingWater => '|',
                        RetainedWater => '~',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", display)
    }
}
