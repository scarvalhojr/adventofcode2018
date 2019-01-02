use std::cmp::min;

type Score = u8;

const PLAYER_1_START: Score = 3;
const PLAYER_2_START: Score = 7;
const SCORE_DIGITS: usize = 10;

pub struct Scoreboard {
    scores: Vec<u8>,
    player1: usize,
    player2: usize,
}

impl Default for Scoreboard {
    fn default() -> Self {
        Scoreboard {
            scores: vec![PLAYER_1_START, PLAYER_2_START],
            player1: 0,
            player2: 1,
        }
    }
}

impl Scoreboard {
    pub fn get_score_after(&mut self, num_recipes: usize) -> String {
        while self.scores.len() < num_recipes + SCORE_DIGITS {
            self.mix_recipes();
        }
        self.scores
            .iter()
            .skip(num_recipes)
            .take(SCORE_DIGITS)
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .concat()
    }

    fn mix_recipes(&mut self) {
        let score1 = self.scores[self.player1];
        let score2 = self.scores[self.player2];
        let combined = score1 + score2;
        if combined >= 10 {
            self.scores.push(1);
            self.scores.push(combined % 10);
        } else {
            self.scores.push(combined);
        }
        self.player1 = (self.player1 + 1 + score1 as usize) % self.scores.len();
        self.player2 = (self.player2 + 1 + score2 as usize) % self.scores.len();
    }

    pub fn find_pattern(&mut self, digits: &str) -> usize {
        let pattern: Vec<Score> = digits
            .chars()
            .map(|d| d.to_digit(10).map(|v| v as Score))
            .collect::<Option<_>>()
            .expect("Invalid input");

        let mut start = 0;
        loop {
            while start < self.scores.len() {
                let len = min(self.scores.len() - start, pattern.len());
                if self.scores[start..start + len] == pattern[..len] {
                    if len == pattern.len() {
                        return start;
                    }
                    break;
                }
                start += 1;
            }
            self.mix_recipes();
        }
    }
}
