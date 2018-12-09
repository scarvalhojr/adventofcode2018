use std::collections::HashMap;

const MAGIC_PLAY: u32 = 23;

type MarbleNum = u32;

struct Marble {
    pub prev: MarbleNum,
    pub next: MarbleNum,
}

pub struct MarbleGame {
    curr_marble: MarbleNum,
    next_marble: MarbleNum,
    curr_player: usize,
    marbles: HashMap<MarbleNum, Marble>,
    scores: Vec<u32>,
}

impl MarbleGame {
    pub fn new(num_players: usize) -> Self {
        MarbleGame {
            curr_marble: 0,
            next_marble: 1,
            curr_player: 0,
            marbles: vec![(0, Marble { prev: 0, next: 0 })]
                .into_iter()
                .collect(),
            scores: vec![0; num_players],
        }
    }

    pub fn play_until(&mut self, last_marble: MarbleNum) {
        while self.curr_marble < last_marble {
            if self.next_marble % MAGIC_PLAY == 0 {
                let points = self.delete_marble();
                self.scores[self.curr_player] += points + self.next_marble;
            } else {
                self.insert_marble();
            }
            self.next_marble += 1;
            self.curr_player = (self.curr_player + 1) % self.scores.len();
        }
    }

    pub fn highest_score(&self) -> u32 {
        *self.scores.iter().max().unwrap_or(&0)
    }

    fn insert_marble(&mut self) {
        let after_marble = self
            .marbles
            .get(&self.curr_marble)
            .map(|marble| marble.next)
            .unwrap();
        let before_marble = self
            .marbles
            .get(&after_marble)
            .map(|marble| marble.next)
            .unwrap();
        let new_marble = self.next_marble;
        self.marbles.insert(
            self.next_marble,
            Marble {
                prev: after_marble,
                next: before_marble,
            },
        );
        self.marbles
            .entry(after_marble)
            .and_modify(|marble| marble.next = new_marble);
        self.marbles
            .entry(before_marble)
            .and_modify(|marble| marble.prev = new_marble);
        self.curr_marble = new_marble;
    }

    fn delete_marble(&mut self) -> u32 {
        let mut delete_marble = self.curr_marble;
        for _ in 1..=7 {
            delete_marble = self
                .marbles
                .get(&delete_marble)
                .map(|marble| marble.prev)
                .unwrap();
        }
        let (before_marble, after_marble) = self
            .marbles
            .get(&delete_marble)
            .map(|marble| (marble.prev, marble.next))
            .unwrap();
        self.marbles
            .entry(before_marble)
            .and_modify(|marble| marble.next = after_marble);
        self.marbles
            .entry(after_marble)
            .and_modify(|marble| marble.prev = before_marble);
        self.marbles.remove(&delete_marble);
        self.curr_marble = after_marble;
        delete_marble
    }
}

pub fn part1(game: &mut MarbleGame, last_marble: MarbleNum) -> u32 {
    game.play_until(last_marble);
    game.highest_score()
}

pub fn part2(game: &mut MarbleGame, last_marble: MarbleNum) -> u32 {
    game.play_until(100 * last_marble);
    game.highest_score()
}
