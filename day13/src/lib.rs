use crate::Direction::*;
use crate::Path::*;
use crate::Turn::*;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub type Position = (i32, i32);

#[derive(Clone)]
enum Path {
    Empty,
    Horizontal,
    Vertical,
    Intersection,
    ForwardCurve,
    BackwardCurve,
}

#[derive(Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Clone)]
pub struct Mine {
    track: HashMap<Position, Path>,
    carts: HashMap<Position, (Direction, Turn)>,
    max_x: i32,
    max_y: i32,
}

impl Mine {
    pub fn first_crash_pos(&mut self) -> Position {
        loop {
            for cart_pos in self.get_cart_positions() {
                if let Some(crash_pos) = self.move_cart(cart_pos) {
                    return crash_pos;
                }
            }
        }
    }

    pub fn last_cart_pos(&mut self) -> Position {
        while self.carts.len() > 1 {
            for cart_pos in self.get_cart_positions() {
                self.move_cart(cart_pos);
            }
        }
        *self.carts.keys().next().expect("All carts crashed")
    }

    fn get_cart_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = self.carts.keys().cloned().collect();
        positions.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        positions
    }

    fn move_cart(&mut self, curr_pos: Position) -> Option<Position> {
        let curr_state = self.carts.remove(&curr_pos)?;

        let new_pos = match curr_state.0 {
            North => (curr_pos.0, curr_pos.1 - 1),
            West => (curr_pos.0 - 1, curr_pos.1),
            East => (curr_pos.0 + 1, curr_pos.1),
            South => (curr_pos.0, curr_pos.1 + 1),
        };

        let new_path = self.track.get(&new_pos).expect("Invalid track");

        let new_dir = match curr_state.0 {
            North => match (new_path, &curr_state.1) {
                (Intersection, Left) => West,
                (Intersection, Straight) => North,
                (Intersection, Right) => East,
                (Vertical, _) => North,
                (ForwardCurve, _) => East,
                (BackwardCurve, _) => West,
                _ => panic!("Cart in invalid position or direction"),
            },
            West => match (new_path, &curr_state.1) {
                (Intersection, Left) => South,
                (Intersection, Straight) => West,
                (Intersection, Right) => North,
                (Horizontal, _) => West,
                (ForwardCurve, _) => South,
                (BackwardCurve, _) => North,
                _ => panic!("Cart in invalid position or direction"),
            },
            East => match (new_path, &curr_state.1) {
                (Intersection, Left) => North,
                (Intersection, Straight) => East,
                (Intersection, Right) => South,
                (Horizontal, _) => East,
                (ForwardCurve, _) => North,
                (BackwardCurve, _) => South,
                _ => panic!("Cart in invalid position or direction"),
            },
            South => match (new_path, &curr_state.1) {
                (Intersection, Left) => East,
                (Intersection, Straight) => South,
                (Intersection, Right) => West,
                (Vertical, _) => South,
                (ForwardCurve, _) => West,
                (BackwardCurve, _) => East,
                _ => panic!("Cart in invalid position or direction"),
            },
        };

        let next_turn = match (new_path, curr_state.1) {
            (Intersection, Left) => Straight,
            (Intersection, Straight) => Right,
            (Intersection, Right) => Left,
            (_, turn) => turn,
        };

        self.carts.remove(&curr_pos);
        match self.carts.entry(new_pos) {
            Entry::Occupied(entry) => {
                // Collision
                entry.remove_entry();
                Some(new_pos)
            }
            Entry::Vacant(entry) => {
                // No collision
                entry.insert((new_dir, next_turn));
                None
            }
        }
    }
}

impl FromStr for Mine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^[ \-\|\+/\\\^v><\n]+$").unwrap();
        if !pattern.is_match(s) {
            return Err("Invalid character in input".to_string());
        }

        let mut carts = HashMap::new();

        let track = s
            .lines()
            .enumerate()
            .map(|(pos_y, line)| (pos_y as i32, line))
            .flat_map(|(pos_y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(pos_x, ch)| (pos_x as i32, ch))
                    .filter_map(|(pos_x, ch)| match ch {
                        '-' => Some(((pos_x, pos_y), Horizontal)),
                        '|' => Some(((pos_x, pos_y), Vertical)),
                        '/' => Some(((pos_x, pos_y), ForwardCurve)),
                        '\\' => Some(((pos_x, pos_y), BackwardCurve)),
                        '+' => Some(((pos_x, pos_y), Intersection)),
                        '>' => {
                            carts.insert((pos_x, pos_y), (East, Left));
                            Some(((pos_x, pos_y), Horizontal))
                        }
                        '<' => {
                            carts.insert((pos_x, pos_y), (West, Left));
                            Some(((pos_x, pos_y), Horizontal))
                        }
                        '^' => {
                            carts.insert((pos_x, pos_y), (North, Left));
                            Some(((pos_x, pos_y), Vertical))
                        }
                        'v' => {
                            carts.insert((pos_x, pos_y), (South, Left));
                            Some(((pos_x, pos_y), Vertical))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>();

        let max_x = 1 + track.keys().map(|(x, _)| *x).max().unwrap();
        let max_y = 1 + track.keys().map(|(_, y)| *y).max().unwrap();

        Ok(Mine {
            track,
            carts,
            max_x,
            max_y,
        })
    }
}

impl Display for Mine {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let display = (0..=self.max_y)
            .map(|y| {
                (0..=self.max_x)
                    .map(|x| match self.carts.get(&(x, y)) {
                        Some((North, _)) => '^',
                        Some((West, _)) => '<',
                        Some((South, _)) => 'v',
                        Some((East, _)) => '>',
                        _ => match self.track.get(&(x, y)).unwrap_or(&Empty) {
                            Horizontal => '-',
                            Vertical => '|',
                            Intersection => '+',
                            ForwardCurve => '/',
                            BackwardCurve => '\\',
                            _ => ' ',
                        },
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", display)
    }
}
