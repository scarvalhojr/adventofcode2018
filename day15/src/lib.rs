use self::Attack::{Hit, Kill};
use self::Element::{Elf, Free, Goblin, Wall};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

type Position = (i32, i32);
type HitPoints = u8;

const DEFAULT_ATTACK_POWER: HitPoints = 3;
const START_HIT_POINTS: HitPoints = 200;

#[derive(Clone)]
enum Element {
    Wall,
    Free,
    Elf(HitPoints),
    Goblin(HitPoints),
}

impl Element {
    fn is_free(&self) -> bool {
        match self {
            Free => true,
            _ => false,
        }
    }

    fn is_elf(&self) -> bool {
        match self {
            Elf(_) => true,
            _ => false,
        }
    }

    fn is_goblin(&self) -> bool {
        match self {
            Goblin(_) => true,
            _ => false,
        }
    }

    fn is_unit(&self) -> bool {
        match self {
            Elf(_) => true,
            Goblin(_) => true,
            _ => false,
        }
    }

    fn is_enemy(&self, other: &Element) -> bool {
        match (self, other) {
            (Elf(_), Goblin(_)) => true,
            (Goblin(_), Elf(_)) => true,
            _ => false,
        }
    }

    fn hit_points(&self) -> HitPoints {
        match self {
            Elf(hp) => *hp,
            Goblin(hp) => *hp,
            _ => 0,
        }
    }

    fn take_hit(&self, power: HitPoints) -> Self {
        match self {
            Elf(hp) => {
                if *hp > power {
                    Elf(hp - power)
                } else {
                    Free
                }
            }
            Goblin(hp) => {
                if *hp > power {
                    Goblin(hp - power)
                } else {
                    Free
                }
            }
            _ => panic!("Cannot attack a free area or a wall"),
        }
    }
}

#[derive(Clone)]
pub struct Battle {
    areas: HashMap<Position, Element>,
    elf_power: HitPoints,
    round: u64,
}

enum Attack {
    Hit,
    Kill,
}

impl Battle {
    pub fn run(&mut self) {
        self.do_rounds(false);
    }

    pub fn elves_win_undefeated(&mut self, elf_power: HitPoints) -> bool {
        self.elf_power = elf_power;
        self.do_rounds(true);
        !self.areas.values().any(|area| area.is_goblin())
    }

    fn do_rounds(&mut self, dying_elf_ends: bool) {
        loop {
            let mut pos: Vec<Position> = self
                .areas
                .iter()
                .filter(|(_, area)| area.is_unit())
                .map(|(pos, _)| pos)
                .cloned()
                .collect();
            pos.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

            for attacker_pos in pos.iter() {
                if self.is_over() {
                    return;
                }
                if let Some(Kill) = self.play_unit(*attacker_pos) {
                    if dying_elf_ends && self.areas[attacker_pos].is_goblin() {
                        // An Elf unit died
                        return;
                    }
                }
            }
            self.round += 1;
        }
    }

    pub fn get_outcome(&self) -> u64 {
        self.round
            * self
                .areas
                .values()
                .map(|area| u64::from(area.hit_points()))
                .sum::<u64>()
    }

    fn play_unit(&mut self, unit_pos: Position) -> Option<Attack> {
        if self.areas.get(&unit_pos).unwrap_or(&Wall).is_unit() {
            match self.move_unit(unit_pos) {
                None => self.attack(unit_pos),
                Some(new_pos) => self.attack(new_pos),
            }
        } else {
            // Unit died before it could play
            None
        }
    }

    fn move_unit(&mut self, attacker_pos: Position) -> Option<Position> {
        let attacker = &self.areas[&attacker_pos];
        let first_moves = [
            // First moves in "reading order": up, left, right, down
            (attacker_pos.0 - 1, attacker_pos.1),
            (attacker_pos.0, attacker_pos.1 - 1),
            (attacker_pos.0, attacker_pos.1 + 1),
            (attacker_pos.0 + 1, attacker_pos.1),
        ];
        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();
        for (priority, target_pos) in first_moves.iter().enumerate() {
            let area = self.areas.get(target_pos).unwrap_or(&Wall);
            if area.is_enemy(attacker) {
                // Unit is in range of an enemy: no need to move
                return None;
            } else if area.is_free() {
                heap.push(Path {
                    dist: 1,
                    priority,
                    first_move: *target_pos,
                    position: *target_pos,
                });
                visited.insert(*target_pos);
            }
        }

        let mut targets = Vec::new();
        let mut min_dist = None;
        while let Some(path) = heap.pop() {
            if min_dist.map_or(false, |min_dist| path.dist >= min_dist) {
                continue;
            }
            let next_positions = [
                (path.position.0 - 1, path.position.1),
                (path.position.0, path.position.1 - 1),
                (path.position.0, path.position.1 + 1),
                (path.position.0 + 1, path.position.1),
            ];
            for target_pos in next_positions.iter() {
                if visited.contains(target_pos) {
                    continue;
                }
                visited.insert(*target_pos);
                let area = self.areas.get(target_pos).unwrap_or(&Wall);
                if area.is_enemy(attacker) {
                    targets.push((*target_pos, path.first_move));
                    min_dist = Some(path.dist + 1);
                } else if area.is_free() {
                    heap.push(Path {
                        dist: path.dist + 1,
                        priority: path.priority,
                        first_move: path.first_move,
                        position: *target_pos,
                    });
                }
            }
        }

        if let Some((_pos, first_move)) = targets.iter().min_by_key(|t| t.0) {
            let attacker = self.areas.remove(&attacker_pos).unwrap();
            self.areas.insert(*first_move, attacker);
            self.areas.insert(attacker_pos, Free);
            Some(*first_move)
        } else {
            // No target found
            None
        }
    }

    fn attack(&mut self, attacker_pos: Position) -> Option<Attack> {
        let attacker = &self.areas[&attacker_pos];
        let targets = [
            // Targets in "reading order": up, left, right, down
            (attacker_pos.0 - 1, attacker_pos.1),
            (attacker_pos.0, attacker_pos.1 - 1),
            (attacker_pos.0, attacker_pos.1 + 1),
            (attacker_pos.0 + 1, attacker_pos.1),
        ];
        if let Some((target_pos, target)) = targets
            .iter()
            .map(|pos| (pos, self.areas.get(&pos).unwrap_or(&Wall)))
            .filter(|(_, target)| attacker.is_enemy(target))
            .min_by_key(|(_, target)| target.hit_points())
        {
            let power = match attacker {
                Elf(_) => self.elf_power,
                _ => DEFAULT_ATTACK_POWER,
            };
            let hit_target = target.take_hit(power);
            let outcome = match hit_target {
                Free => Some(Kill),
                _ => Some(Hit),
            };
            self.areas.insert(*target_pos, hit_target);
            outcome
        } else {
            // No target within reach
            None
        }
    }

    fn is_over(&self) -> bool {
        !self.areas.values().any(|area| area.is_elf())
            || !self.areas.values().any(|area| area.is_goblin())
    }
}

pub fn part2(battle: &Battle) -> u64 {
    for power in DEFAULT_ATTACK_POWER.. {
        let mut new_battle = battle.clone();
        if new_battle.elves_win_undefeated(power) {
            return new_battle.get_outcome();
        }
    }
    0
}

#[derive(Eq)]
struct Path {
    dist: u64,
    priority: usize,
    first_move: Position,
    position: Position,
}

impl Ord for Path {
    fn cmp(&self, other: &Path) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then(other.priority.cmp(&self.priority))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        self.dist == other.dist
            && self.priority == other.priority
            && self.position == other.position
    }
}

impl FromStr for Battle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^[\.#EG\n]+$").unwrap();
        if !pattern.is_match(s) {
            return Err("Invalid character in input".to_string());
        }

        let areas = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(col, ch)| match ch {
                        '.' => Some(((row as i32, col as i32), Free)),
                        'E' => Some((
                            (row as i32, col as i32),
                            Elf(START_HIT_POINTS),
                        )),
                        'G' => Some((
                            (row as i32, col as i32),
                            Goblin(START_HIT_POINTS),
                        )),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>();

        Ok(Self {
            areas,
            elf_power: DEFAULT_ATTACK_POWER,
            round: 0,
        })
    }
}

impl Display for Battle {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let min_row = self.areas.keys().map(|pos| pos.0).min().unwrap_or(0);
        let max_row = self.areas.keys().map(|pos| pos.0).max().unwrap_or(0);
        let min_col = self.areas.keys().map(|pos| pos.1).min().unwrap_or(0);
        let max_col = self.areas.keys().map(|pos| pos.1).max().unwrap_or(0);
        let display: String = (min_row - 1..=max_row + 1)
            .map(|row| {
                (min_col - 1..=max_col + 1)
                    .map(|col| {
                        match self.areas.get(&(row, col)).unwrap_or(&Wall) {
                            Free => '.',
                            Elf(_) => 'E',
                            Goblin(_) => 'G',
                            _ => '#',
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", display)
    }
}
