use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::num::ParseIntError;
use std::str::FromStr;

type Point = (i64, i64, i64);

#[derive(PartialEq)]
pub struct Nanobot {
    center: Point,
    radius: i64,
}

impl Nanobot {
    fn in_range(&self, other: &Nanobot) -> bool {
        (self.center.0 - other.center.0).abs()
            + (self.center.1 - other.center.1).abs()
            + (self.center.2 - other.center.2).abs()
            <= self.radius
    }
}

pub fn part1(bots: &[Nanobot]) -> usize {
    if let Some(strongest) = bots.iter().max_by_key(|bot| bot.radius) {
        bots.iter().filter(|&bot| strongest.in_range(bot)).count()
    } else {
        0
    }
}

type Dimension = (i64, i64, i64);

#[derive(Eq, Debug, PartialEq)]
struct Area {
    min_coord: Point,
    dimension: Dimension,
    bot_count: usize,
}

impl Area {
    fn new(min_coord: Point, dimension: Dimension) -> Self {
        assert!(dimension.0 > 0 && dimension.1 > 0 && dimension.2 > 0);
        Area {
            min_coord,
            dimension,
            bot_count: 0,
        }
    }

    fn get_enclosing_area(bots: &[Nanobot]) -> Self {
        if bots.is_empty() {
            return Area::new((0, 0, 0), (1, 1, 1));
        }

        let mut min_0 = bots[0].center.0 - bots[0].radius;
        let mut min_1 = bots[0].center.1 - bots[0].radius;
        let mut min_2 = bots[0].center.2 - bots[0].radius;
        let mut max_0 = bots[0].center.0 + bots[0].radius;
        let mut max_1 = bots[0].center.1 + bots[0].radius;
        let mut max_2 = bots[0].center.2 + bots[0].radius;
        for bot in bots.iter().skip(1) {
            min_0 = min_0.min(bot.center.0 - bot.radius);
            min_1 = min_1.min(bot.center.1 - bot.radius);
            min_2 = min_2.min(bot.center.2 - bot.radius);
            max_0 = max_0.max(bot.center.0 + bot.radius);
            max_1 = max_1.max(bot.center.1 + bot.radius);
            max_2 = max_2.max(bot.center.2 + bot.radius);
        }

        let dim_0 = 1 + (max_0 - min_0).abs();
        let dim_1 = 1 + (max_1 - min_1).abs();
        let dim_2 = 1 + (max_2 - min_2).abs();
        Area::new((min_0, min_1, min_2), (dim_0, dim_1, dim_2))
    }

    fn scan_bots(&mut self, bots: &[Nanobot]) {
        self.bot_count = bots.iter().filter(|bot| self.in_range(bot)).count();
    }

    fn in_range(&self, bot: &Nanobot) -> bool {
        [
            (bot.center.0, self.min_coord.0, self.dimension.0),
            (bot.center.1, self.min_coord.1, self.dimension.1),
            (bot.center.2, self.min_coord.2, self.dimension.2),
        ]
        .iter()
        .map(|(bot_center, area_min, area_dim)| {
            if bot_center < area_min {
                area_min - bot_center
            } else {
                0.max(bot_center - area_min - area_dim + 1)
            }
        })
        .sum::<i64>()
            <= bot.radius
    }

    fn dist_orig(&self) -> i64 {
        [
            (self.min_coord.0, self.dimension.0),
            (self.min_coord.1, self.dimension.1),
            (self.min_coord.2, self.dimension.2),
        ]
        .iter()
        .map(|&(min_coord, dim)| {
            if min_coord <= 0 && min_coord + dim >= 0 {
                0
            } else if min_coord < 0 {
                (min_coord + dim).abs()
            } else {
                min_coord
            }
        })
        .sum()
    }

    fn min_dimension(&self) -> i64 {
        self.dimension.0.min(self.dimension.1.min(self.dimension.2))
    }

    fn is_point(&self) -> bool {
        self.dimension.0 == 1 && self.dimension.1 == 1 && self.dimension.2 == 1
    }

    fn split(&self) -> SubAreas {
        SubAreas::split_area(self)
    }
}

struct SubAreas {
    split_0: Vec<(i64, i64)>,
    split_1: Vec<(i64, i64)>,
    split_2: Vec<(i64, i64)>,
    idx_0: usize,
    idx_1: usize,
    idx_2: usize,
}

impl SubAreas {
    fn split_area(area: &Area) -> Self {
        let split_coord = |min_coord, dim| {
            if dim > 1 {
                let half_dim = dim / 2;
                vec![
                    (min_coord, half_dim),
                    (min_coord + half_dim, dim - half_dim),
                ]
            } else {
                vec![(min_coord, dim)]
            }
        };

        let split_0 = split_coord(area.min_coord.0, area.dimension.0);
        let split_1 = split_coord(area.min_coord.1, area.dimension.1);
        let split_2 = split_coord(area.min_coord.2, area.dimension.2);

        SubAreas {
            split_0,
            split_1,
            split_2,
            idx_0: 0,
            idx_1: 0,
            idx_2: 0,
        }
    }
}

impl Iterator for SubAreas {
    type Item = Area;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx_0 >= self.split_0.len() {
            return None;
        }

        let split_0 = self.split_0[self.idx_0];
        let split_1 = self.split_1[self.idx_1];
        let split_2 = self.split_2[self.idx_2];

        let next_area = Area::new(
            (split_0.0, split_1.0, split_2.0),
            (split_0.1, split_1.1, split_2.1),
        );

        self.idx_2 += 1;
        if self.idx_2 >= self.split_2.len() {
            self.idx_2 = 0;
            self.idx_1 += 1;
            if self.idx_1 >= self.split_1.len() {
                self.idx_1 = 0;
                self.idx_0 += 1;
            }
        }

        Some(next_area)
    }
}

impl Ord for Area {
    fn cmp(&self, other: &Area) -> Ordering {
        self.bot_count
            .cmp(&other.bot_count)
            .then(self.dist_orig().cmp(&other.dist_orig()).reverse())
            .then(self.min_dimension().cmp(&other.min_dimension()).reverse())
            .then(self.min_coord.0.cmp(&other.min_coord.0))
            .then(self.min_coord.1.cmp(&other.min_coord.1))
            .then(self.min_coord.2.cmp(&other.min_coord.2))
    }
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Area) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part2(bots: &[Nanobot]) -> i64 {
    let mut heap = BinaryHeap::new();
    heap.push(Area::get_enclosing_area(bots));

    while let Some(area) = heap.pop() {
        if area.is_point() {
            return area.dist_orig();
        }
        for mut sub_area in area.split() {
            sub_area.scan_bots(bots);
            heap.push(sub_area);
        }
    }

    // It should never get here
    panic!("Solution not found.");
}

impl FromStr for Nanobot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern =
            Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
        let groups = pattern.captures(s).ok_or("Invalid format")?;
        let numbers: Vec<i64> = groups
            .iter()
            .skip(1)
            .map(|val| {
                val.unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())
            })
            .collect::<Result<_, _>>()?;

        Ok(Nanobot {
            center: (numbers[0], numbers[1], numbers[2]),
            radius: numbers[3],
        })
    }
}
