use regex::Regex;
use std::collections::HashMap;

pub struct Rooms {
    distance: HashMap<(i32, i32), i32>,
}

impl Rooms {
    pub fn build(directions: &str) -> Result<Rooms, String> {
        let pattern = Regex::new(r"\^([NEWS\|\(\)]+)\$").unwrap();
        let groups = pattern.captures(directions).ok_or("Invalid input")?;
        let steps = groups.iter().nth(1).unwrap().unwrap().as_str().chars();

        let mut stack: Vec<(i32, i32)> = Vec::new();
        let mut distance = HashMap::new();
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut dist = 0;
        distance.insert((x_pos, y_pos), 0);

        for step in steps {
            match step {
                '(' => {
                    stack.push((x_pos, y_pos));
                }
                '|' => {
                    let last = stack.last().ok_or("Unexpected direction")?;
                    x_pos = last.0;
                    y_pos = last.1;
                    dist = distance[&(x_pos, y_pos)];
                }
                ')' => {
                    let back = stack.pop().ok_or("Unexpected direction")?;
                    x_pos = back.0;
                    y_pos = back.1;
                    dist = distance[&(x_pos, y_pos)];
                }
                _ => {
                    match step {
                        'E' => x_pos += 2,
                        'W' => x_pos -= 2,
                        'N' => y_pos -= 2,
                        _ => y_pos += 2,
                    }
                    dist += 1;
                    distance.entry((x_pos, y_pos)).or_insert(dist);
                }
            }
        }

        Ok(Rooms { distance })
    }
}

pub fn part1(rooms: &Rooms) -> i32 {
    *rooms.distance.values().max().unwrap_or(&0)
}

pub fn part2(rooms: &Rooms) -> usize {
    rooms.distance.values().filter(|&d| *d >= 1000).count()
}
