use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use Army::*;
use Attack::*;

#[derive(Clone, PartialEq)]
enum Army {
    ImmuneSystem,
    Infection,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Attack {
    Slashing,
    Fire,
    Cold,
    Bludgeoning,
    Radiation,
}

#[derive(Clone)]
struct Group {
    army: Army,
    units: u32,
    hit_points: u32,
    weakness: HashSet<Attack>,
    immunity: HashSet<Attack>,
    attack: Attack,
    damage: u32,
    initiative: u8,
}

#[derive(Clone)]
pub struct Combat {
    groups: Vec<Group>,
}

struct Fight {
    attacker_index: usize,
    attacker_initiative: u8,
    defender_index: usize,
}

impl Combat {
    fn run(&mut self) {
        loop {
            let mut units_killed = false;
            for fight in self.select_fights() {
                let attacker = &self.groups[fight.attacker_index];
                if !attacker.is_alive() {
                    // Attacker died before its turn
                    continue;
                }
                let power = attacker.power();
                let attack = attacker.attack;
                let defender = &mut self.groups[fight.defender_index];
                if defender.take_hit(power, attack) {
                    units_killed = true;
                }
            }
            if !units_killed {
                // No unit was killed so combat is over
                return;
            }
        }
    }

    fn total_units(&self) -> u32 {
        self.groups.iter().map(|group| group.units).sum()
    }

    fn damage_boost(&mut self, army: Army, boost: u32) {
        for group in self.groups.iter_mut() {
            if group.army == army {
                group.damage_boost(boost);
            }
        }
    }

    fn select_fights(&self) -> Vec<Fight> {
        let mut attackers: Vec<_> = self
            .groups
            .iter()
            .enumerate()
            .filter(|(_, group)| group.is_alive())
            .collect();

        attackers.sort_unstable_by(|(_, attacker_a), (_, attacker_b)| {
            attacker_a
                .power()
                .cmp(&attacker_b.power())
                .then(attacker_a.initiative.cmp(&attacker_b.initiative))
                .reverse()
        });

        let mut defenders: HashMap<_, _> = self
            .groups
            .iter()
            .enumerate()
            .filter(|(_, group)| group.is_alive())
            .collect();

        let mut fights = attackers
            .iter()
            .filter_map(|&(attacker_index, attacker)| {
                defenders
                    .iter()
                    .filter(|(_, &defender)| {
                        attacker.army != defender.army
                            && !defender.is_immune_to(attacker.attack)
                    })
                    .max_by(|(_, def_a), (_, def_b)| {
                        def_a
                            .damage_multiplier(attacker.attack)
                            .cmp(&def_b.damage_multiplier(attacker.attack))
                            .then(def_a.power().cmp(&def_b.power()))
                            .then(def_a.initiative.cmp(&def_b.initiative))
                    })
                    .map(|(defender_index, _)| *defender_index)
                    .map(|defender_index| {
                        defenders.remove(&defender_index);
                        Fight {
                            attacker_index,
                            attacker_initiative: attacker.initiative,
                            defender_index,
                        }
                    })
            })
            .collect::<Vec<_>>();

        fights.sort_unstable_by(|fight_a, fight_b| {
            fight_a
                .attacker_initiative
                .cmp(&fight_b.attacker_initiative)
                .reverse()
        });
        fights
    }

    fn get_winner(&self) -> Option<Army> {
        let immune_system_alive = self
            .groups
            .iter()
            .any(|group| group.army == ImmuneSystem && group.is_alive());
        let infection_alive = self
            .groups
            .iter()
            .any(|group| group.army == Infection && group.is_alive());

        if immune_system_alive && !infection_alive {
            Some(ImmuneSystem)
        } else if infection_alive && !immune_system_alive {
            Some(Infection)
        } else {
            None
        }
    }
}

impl Group {
    fn is_alive(&self) -> bool {
        self.units > 0
    }

    fn is_immune_to(&self, attack: Attack) -> bool {
        self.immunity.contains(&attack)
    }

    fn damage_multiplier(&self, attack: Attack) -> u32 {
        if self.immunity.contains(&attack) {
            0
        } else if self.weakness.contains(&attack) {
            2
        } else {
            1
        }
    }

    fn damage_boost(&mut self, boost: u32) {
        self.damage += boost;
    }

    fn power(&self) -> u32 {
        self.units * self.damage
    }

    fn take_hit(&mut self, power: u32, attack: Attack) -> bool {
        let kills = (power * self.damage_multiplier(attack)) / self.hit_points;
        if kills == 0 {
            false
        } else if kills < self.units {
            self.units -= kills;
            true
        } else if self.units > 0 {
            self.units = 0;
            true
        } else {
            false
        }
    }
}

pub fn part1(combat: &Combat) -> u32 {
    let mut new_combat = combat.clone();
    new_combat.run();
    new_combat.total_units()
}

pub fn part2(combat: &Combat) -> u32 {
    let mut boost = 1;
    loop {
        let mut new_combat = combat.clone();
        new_combat.damage_boost(ImmuneSystem, boost);
        new_combat.run();
        if let Some(ImmuneSystem) = new_combat.get_winner() {
            return new_combat.total_units();
        }
        boost += 1;
    }
}

impl FromStr for Combat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let combat_regex =
            Regex::new(r"^Immune System:\n((?s).*)\nInfection:\n((?s).*)$")
                .unwrap();
        let captures =
            combat_regex.captures(s).ok_or("Invalid combat input")?;
        let groups = captures
            .iter()
            .enumerate()
            .skip(1)
            .flat_map(|(army_num, capture)| {
                let army = match army_num {
                    1 => ImmuneSystem,
                    _ => Infection,
                };
                capture
                    .unwrap()
                    .as_str()
                    .lines()
                    .map(|line| Group::parse_group(line, army.clone()))
                    .collect::<Vec<_>>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Combat { groups })
    }
}

impl FromStr for Attack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "slashing" => Ok(Slashing),
            "fire" => Ok(Fire),
            "cold" => Ok(Cold),
            "bludgeoning" => Ok(Bludgeoning),
            "radiation" => Ok(Radiation),
            _ => Err(format!("Unknown attack type {}", s)),
        }
    }
}

impl Group {
    fn parse_group(s: &str, army: Army) -> Result<Self, String> {
        let group_regex = Regex::new(concat!(
            r"^(?P<units>\d+) units each with (?P<hits>\d+) hit points ",
            r"(?P<defense>.*)?",
            r"with an attack that does (?P<damage>\d+) (?P<attack>\w+?) ",
            r"damage at initiative (?P<initiative>\d+)$",
        ))
        .unwrap();
        let captures = group_regex.captures(s).ok_or("Invalid group input")?;

        let units = captures
            .name("units")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .map_err(|err| err.to_string())?;
        let hit_points = captures
            .name("hits")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .map_err(|err| err.to_string())?;

        let mut weakness = HashSet::new();
        let mut immunity = HashSet::new();

        if let Some(defense) = captures.name("defense") {
            let weak_regex = Regex::new(r"weak to ([\w ,]+)").unwrap();
            if let Some(weak_cap) = weak_regex.captures(defense.as_str()) {
                weakness = weak_cap
                    .iter()
                    .nth(1)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.parse())
                    .collect::<Result<HashSet<_>, _>>()?;
            }
            let immune_regex = Regex::new(r"immune to ([\w ,]+)").unwrap();
            if let Some(immune_cap) = immune_regex.captures(defense.as_str()) {
                immunity = immune_cap
                    .iter()
                    .nth(1)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.parse())
                    .collect::<Result<HashSet<_>, _>>()?;
            }
        }

        let attack = captures.name("attack").unwrap().as_str().parse()?;
        let damage = captures
            .name("damage")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .map_err(|err| err.to_string())?;
        let initiative = captures
            .name("initiative")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .map_err(|err| err.to_string())?;

        Ok(Group {
            army,
            units,
            hit_points,
            weakness,
            immunity,
            attack,
            damage,
            initiative,
        })
    }
}
