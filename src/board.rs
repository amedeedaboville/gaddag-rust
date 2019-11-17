use crate::utils::*;
use std::fmt;
use std::collections::HashMap;

pub struct Board {
    state: [[char; 15]; 15],
    dictionary: Dictionary
}

/*
#: TWS
^: DWS
+: TLS
-: DLS
*: center
*/

impl Board {
    pub fn default() -> Board {
        Board { state: [
            ['#', '.', '.', '-', '.', '.', '.', '#', '.', '.', '.', '-', '.', '.', '#'],
            ['.', '^', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '^', '.'],
            ['.', '.', '^', '.', '.', '.', '-', '.', '-', '.', '.', '.', '^', '.', '.'],
            ['-', '.', '.', '^', '.', '.', '.', '-', '.', '.', '.', '^', '.', '.', '-'],
            ['.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.'],
            ['.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.'],
            ['.', '.', '-', '.', '.', '.', '-', '.', '-', '.', '.', '.', '-', '.', '.'],
            ['#', '.', '.', '-', '.', '.', '.', '*', '.', '.', '.', '-', '.', '.', '#'],
            ['.', '.', '-', '.', '.', '.', '-', '.', '-', '.', '.', '.', '-', '.', '.'],
            ['.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.'],
            ['.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.'],
            ['-', '.', '.', '^', '.', '.', '.', '-', '.', '.', '.', '^', '.', '.', '-'],
            ['.', '.', '^', '.', '.', '.', '-', '.', '-', '.', '.', '.', '^', '.', '.'],
            ['.', '^', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '^', '.'],
            ['#', '.', '.', '-', '.', '.', '.', '#', '.', '.', '.', '-', '.', '.', '#'],
        ], dictionary: Dictionary::default() }
    }

    pub fn at_position(&self, p: Position) -> char {
        self.state[p.row][p.col]
    }

    fn is_letter(&self, p: Position) -> bool {
        return !"#^+_*.".contains(self.at_position(p))
    }

    fn set(&mut self, p: Position, c: char) {
        self.state[p.row][p.col] = c;
    }

    pub fn play_word(&mut self, p: Position, word: String, dir: Direction) -> bool {
        let mut current = p.clone();

        for c in word.chars() {
            match self.at_position(current) {
                '.' | '*' | '-' | '+' | '^' | '#' => self.set(current, c),
                                                _ => return false
            }

            if !(current.tick(dir)) { return false }
        }

        true
    }

    pub fn valid_at(&mut self, p: Position) -> [bool; 26] {
        if self.is_letter(p) {
            return [false; 26];
        }

        let mut cross = [false; 26];

        for (i, l) in alph.chars().enumerate() {
            let old = self.at_position(p);
            self.set(p, l);
            cross[i] = self.valid();
            self.set(p, old);
        }

        cross
    }

    pub fn get_words(&self) -> Vec<String> {
        let mut result = Vec::new();

        let mut marked: Vec<Position> = Vec::new();

        for (r, row) in self.state.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                let p = Position { row: r, col: c };
                if !marked.contains(&p) && self.is_letter(p) {
                    // start word finding
                    for d in Direction::iter() {
                        let mut curr = p.clone();
                        let mut word = String::new();
                        while self.is_letter(curr) {
                            word.push(self.at_position(curr));
                            marked.push(curr);
                            curr.tick(*d);
                        }
                        result.push(word);
                    }
                }
            }
        }

        result
    }

    pub fn valid(&self) -> bool {
        self.get_words().iter().all(|x| self.dictionary.check_word(x.to_string()))
    }
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = "-".repeat(66);

        write!(f, "{}\n", sep).expect("fail");
        write!(f, "|    |").expect("fail");
        for row in alph.chars().take(15) {
            write!(f, "{}", format!(" {} |", row)).expect("fail");
        }
        write!(f, "\n{}\n", sep).expect("fail");

        for (num, row) in self.state.iter().enumerate() {
            write!(f, "| {} |", format!("{:0>2}", num+1)).expect("fail");
            for sq in row.iter() {
                match sq {
                    '#' => write!(f, "TWS").expect("fail"),
                    '^' => write!(f, "DWS").expect("fail"),
                    '+' => write!(f, "TLS").expect("fail"),
                    '-' => write!(f, "DLS").expect("fail"),
                    '.' => write!(f, "   ").expect("fail"),
                     _  => write!(f, " {} ", sq).expect("fail")
                };
                write!(f, "|").expect("fail");
            }
            write!(f, "\n{}\n", sep).expect("fail");
        }

        write!(f, "\n")
	}
}
