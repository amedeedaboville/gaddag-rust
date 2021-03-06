use crate::game::Game;
use crate::utils::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::vec::Vec;
use termion::color;

#[derive(Debug)]
pub struct Bag {
    alph: [char; 27],
    amts: [usize; 27],
    values: [i32; 27],
    scores: HashMap<char, i32>,
    pub distribution: Vec<char>,
    random: bool,
}

impl Bag {
    pub fn default() -> Bag {
        let mut bag = Bag {
            alph: [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '?',
            ],
            amts: [
                9, 2, 2, 4, 12, 2, 3, 2, 9, 1, 1, 4, 2, 6, 8, 2, 1, 6, 4, 6, 4, 2, 2, 1, 2, 1, 2,
            ],
            values: [
                1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10, 0,
            ],
            scores: HashMap::new(),
            distribution: Vec::new(),
            random: true,
        };

        for (i, &c) in bag.alph.iter().enumerate() {
            bag.scores.insert(c, bag.values[i]);
        }

        for (i, &c) in bag.alph.iter().enumerate() {
            for _ in 0..bag.amts[i] {
                bag.distribution.push(c);
            }
        }

        bag.distribution.shuffle(&mut rand::thread_rng());

        bag
    }

    pub fn with(order: &Vec<char>) -> Bag {
        let mut b = Bag::default();
        b.distribution = order.to_vec();
        b.random = false;
        b
    }

    pub fn score(&self, c: char) -> i32 {
        match self.scores.get(&c) {
            Some(i) => *i,
            None => 0,
        }
    }

    pub fn draw_tiles(&mut self, n: usize) -> Vec<char> {
        let tiles: Vec<char>;
        if self.random {
            tiles = self
                .distribution
                .choose_multiple(&mut rand::thread_rng(), n)
                .cloned()
                .collect();
        } else {
            tiles = self.distribution.iter().take(n).cloned().collect();
        }
        for i in tiles.iter() {
            self.distribution._remove_item(*i);
        }
        tiles
    }

    pub fn to_str(&self) -> String {
        let mut res = format!("┌─────{:<03}/100─────┐\n", self.distribution.len());

        for (i, c) in self.alph.iter().enumerate() {
            let count = self.distribution.count(*c);

            res = format!(
                "{}│ {}{grey}{used}{spaces}{clear} │\n",
                res,
                c.to_string().repeat(count),
                grey = color::Fg(color::Rgb(220, 220, 220)),
                used = c.to_string().repeat(self.amts[i] - count),
                spaces = &" ".repeat(15 - self.amts[i]),
                clear = RESET
            );
        }

        res = format!("{}└{}┘", res, "─".repeat(17));

        res
    }

    pub fn to_str_for_current_player(&self, game: &Game) -> String {
        let mut d = self.distribution.clone();
        for i in game.get_player(((game.current + 1) % 2) as i32).rack.iter() {
            d.push(*i);
        }

        Bag::with(&d).to_str()
    }
}
