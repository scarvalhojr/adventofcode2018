use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

type Region = u8;
const ROCKY: Region = 0;
const WET: Region = 1;
const NARROW: Region = 2;

type Tool = u8;
const NEITHER: Tool = 0;
const CLIMBING: Tool = 1;
const TORCH: Tool = 2;

const X_MULT: i32 = 16807;
const Y_MULT: i32 = 48271;
const EROSION_MOD: i32 = 20183;
const START_TOOL: Tool = TORCH;
const END_TOOL: Tool = TORCH;

pub struct Cave {
    depth: i32,
    target_x: i32,
    target_y: i32,
    erosion: HashMap<(i32, i32), i32>,
}

impl Cave {
    pub fn new(depth: u32, target_x: u32, target_y: u32) -> Self {
        Cave {
            depth: depth as i32,
            target_x: target_x as i32,
            target_y: target_y as i32,
            erosion: HashMap::new(),
        }
    }

    pub fn total_risk(&mut self) -> u32 {
        (0..=self.target_x)
            .map(|pos_x| {
                (0..=self.target_y)
                    .map(|pos_y| u32::from(self.get_region(pos_x, pos_y)))
                    .sum::<u32>()
            })
            .sum()
    }

    pub fn min_distance(&mut self) -> u32 {
        let mut dist: HashMap<(i32, i32, Tool), u32> = HashMap::new();
        let mut heap: BinaryHeap<Position> = BinaryHeap::new();
        dist.insert((0, 0, START_TOOL), 0);
        heap.push(Position::new(0, 0, START_TOOL, 0));

        while let Some(pos) = heap.pop() {
            if pos.is_target(self.target_x, self.target_y, END_TOOL) {
                return pos.dist;
            }

            if let Some(d) = dist.get(&(pos.x, pos.y, pos.tool)) {
                if *d < pos.dist {
                    continue;
                }
            }

            for next in self.next_positions(&pos) {
                match dist.entry((next.x, next.y, next.tool)) {
                    Entry::Vacant(entry) => {
                        entry.insert(next.dist);
                    }
                    Entry::Occupied(mut entry) => {
                        if *entry.get() <= next.dist {
                            continue;
                        }
                        *entry.get_mut() = next.dist;
                    }
                };
                heap.push(next);
            }
        }
        panic!("Path to target not found.");
    }

    fn get_erosion(&mut self, pos_x: i32, pos_y: i32) -> i32 {
        assert!(pos_x >= 0 && pos_y >= 0);

        if let Some(erosion) = self.erosion.get(&(pos_x, pos_y)) {
            return *erosion;
        }

        let geo_idx = if pos_x == 0 {
            pos_y * Y_MULT
        } else if pos_y == 0 {
            pos_x * X_MULT
        } else if pos_x == self.target_x && pos_y == self.target_y {
            0
        } else {
            self.get_erosion(pos_x - 1, pos_y)
                * self.get_erosion(pos_x, pos_y - 1)
        };

        let erosion = (geo_idx + self.depth) % EROSION_MOD;
        self.erosion.insert((pos_x, pos_y), erosion);
        erosion
    }

    fn get_region(&mut self, pos_x: i32, pos_y: i32) -> Region {
        (self.get_erosion(pos_x, pos_y) % 3) as Region
    }

    fn next_positions(&mut self, pos: &Position) -> Vec<Position> {
        let next_tool = match (self.get_region(pos.x, pos.y), pos.tool) {
            (ROCKY, CLIMBING) => TORCH,
            (ROCKY, TORCH) => CLIMBING,
            (WET, NEITHER) => CLIMBING,
            (WET, CLIMBING) => NEITHER,
            (NARROW, NEITHER) => TORCH,
            (NARROW, TORCH) => NEITHER,
            (region, tool) => panic!(format!(
                "Invalid tool {} for region {} at {},{}",
                tool, region, pos.x, pos.y
            )),
        };

        let can_use = |region, tool| match region {
            ROCKY => tool == CLIMBING || tool == TORCH,
            WET => tool == NEITHER || tool == CLIMBING,
            NARROW => tool == NEITHER || tool == TORCH,
            _ => panic!(format!("Invalid region type {}", region)),
        };

        [
            (pos.x + 1, pos.y, pos.tool, pos.dist + 1),
            (pos.x - 1, pos.y, pos.tool, pos.dist + 1),
            (pos.x, pos.y + 1, pos.tool, pos.dist + 1),
            (pos.x, pos.y - 1, pos.tool, pos.dist + 1),
            (pos.x, pos.y, next_tool, pos.dist + 7),
        ]
        .iter()
        .filter(|(x, y, tool, _)| {
            *x >= 0 && *y >= 0 && can_use(self.get_region(*x, *y), *tool)
        })
        .map(|(x, y, tool, dist)| Position::new(*x, *y, *tool, *dist))
        .collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    tool: Tool,
    dist: u32,
}

impl Position {
    fn new(x: i32, y: i32, tool: Tool, dist: u32) -> Self {
        Position { x, y, tool, dist }
    }

    fn is_target(&self, x: i32, y: i32, tool: Tool) -> bool {
        self.x == x && self.y == y && self.tool == tool
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        // Flip the ordering here since we want a min-heap
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_x = self.erosion.keys().map(|pos| pos.0).max().unwrap_or(0);
        let max_y = self.erosion.keys().map(|pos| pos.1).max().unwrap_or(0);
        let display = (0..=max_y)
            .map(|pos_y| {
                (0..=max_x)
                    .map(|pos_x| match self.erosion.get(&(pos_x, pos_y)) {
                        None => '?',
                        Some(erosion) => match (erosion % 3) as Region {
                            ROCKY => '.',
                            WET => '=',
                            NARROW => '|',
                            _ => '?',
                        },
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", display)
    }
}
