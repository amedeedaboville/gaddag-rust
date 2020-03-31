use crate::board::{Board, S, STATE};
use crate::player::Player;
use crate::utils::Move;

use array_init::array_init;

use std::vec::Vec;

pub struct Game {
    players: [Player; 2],
    board: Board,
    pub current: usize,
    turn: u32,
    pub finished: bool,
    states: Vec<(S, Move, Vec<char>)>,
    pub state: usize,
}

impl Game {
    pub fn default() -> Game {
        let mut board = Board::default();
        let player_1 = Player {
            rack: board.bag.draw_tiles(7),
            name: "p1".to_string(),
            score: 0,
        };
        let player_2 = Player {
            rack: board.bag.draw_tiles(7),
            name: "p2".to_string(),
            score: 0,
        };
        let players = [player_1, player_2];

        Game {
            players,
            board,
            current: 0,
            turn: 1,
            finished: false,
            states: Vec::new(),
            state: 0,
        }
    }

    pub fn set_board(&mut self, board: [[char; 15]; 15]) {
        // for simulation
        self.board.set_board(board);
    }

    pub fn do_move(&mut self) -> (Move, String, String, usize) {
        let r = self.current_player().rack.clone();
        let m = self.players[self.current].do_move(&mut self.board);
        self.states
            .push((self.board.save_state(), Move::of(&m.0), r));
        self.current = (self.current + 1) % 2;
        if self.current == 0 {
            self.turn += 1;
        }
        self.state += 1;
        m
    }

    pub fn finish(&mut self) -> (String, i32, i32) {
        let mut n = 0;
        if self.get_player(1).rack.len() == 0 {
            n = 1;
        }

        let mut end = 0;
        let mut end_s = String::new();

        for s in self.get_player((n + 1) % 2).rack.clone() {
            end += self.board.bag.score(s);
            end_s.push(s);
        }

        end *= 2;
        let p = &mut self.players[n as usize];
        p.score += end as u32;

        self.finished = true;

        (end_s, end, n)
    }

    pub fn is_over(&self) -> bool {
        !(self.board.bag.distribution.len() > 0
            || (self.players[0].rack.len() > 0 && self.players[1].rack.len() > 0))
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
    pub fn get_turn(&self) -> u32 {
        self.turn
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current]
    }

    pub fn get_player(&self, n: i32) -> &Player {
        &self.players[n as usize]
    }

    pub fn get_player_mut(&mut self, n: i32) -> &mut Player {
        &mut self.players[n as usize]
    }

    pub fn set_state(&mut self, to: usize) -> (Move, Vec<char>) {
        let (s, m, r) = &self.states[to];
        self.board.set_state(s);
        self.state = to;
        (Move::of(m), r.clone())
    }

    pub fn get_last_state(&self) -> S {
        if self.state == 0 {
            return (
                STATE,
                vec![],
                [array_init(|_| Vec::new()), array_init(|_| Vec::new())],
            );
        }

        self.states[self.state - 1].0.clone()
    }

    pub fn reset(&mut self) {
        self.board.reset();
        for p in &mut self.players {
            p.score = 0;
            p.rack = self.board.bag.draw_tiles(7);
        }
        self.current = 0;
        self.turn = 1;
        self.finished = false;
        self.states = Vec::new();
        self.state = 0;
    }
}
