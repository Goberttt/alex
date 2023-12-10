use crate::Move;
use crate::move_from_str;

pub struct Board {
    pub walls: [[[bool; 8]; 8]; 2],
    pub players: [[usize; 2]; 2],
    pub to_move: usize,
}
use crate::Move::*;

impl Board {
    pub fn new() -> Board {
        Board {
            walls: [[[false; 8]; 8]; 2],
            players: [[0,4],[8,4]],
            to_move: 0
        }
    }
    pub fn mv(&mut self, m: Move) {
//todo illegal move detection
        match m {
            Step(mut d) => {

                let mut code: Vec<usize> = vec![];
                while {
                    code.push(d % 10);
                    d /= 10;
                    d != 0
                } {};
                for i in code.iter() {
                    match i {
                    1 => self.players[self.to_move][0] -= 1, //N
                    2 => self.players[self.to_move][1] += 1, //E
                    3 => self.players[self.to_move][0] += 1, //S
                    4 => self.players[self.to_move][1] -= 1, //W
                    _ => (), //todo illegal input
                    }

                }
            },
 //                   11 => self.players[self.to_move][0] -= 2, //NN
 //                   12 => {self.players[self.to_move][0] -= 1; self.players[self.to_move][1] += 1}, //NE
 //                   14 => {self.players[self.to_move][0] -= 1; self.players[self.to_move][1] -= 1}, //NW
 //                   33 => self.players[self.to_move][0] += 2, //SS
 //                   32 => {self.players[self.to_move][0] += 1; self.players[self.to_move][1] += 1}, //SE
 //                   34 => {self.players[self.to_move][0] += 1; self.players[self.to_move][1] -= 1}, //SW
 //                   _ => (),
            Wall([i, y, x]) => self.walls[i][y][x] = true,
        }
        if self.to_move == 0 {self.to_move = 1} else {self.to_move = 0};
    }
    pub fn from(s: &str) -> Result<Board, String> {
        let mut b = Board::new();
        for m in s.split_whitespace() {
            match move_from_str(m) {
                Ok(mv) => b.mv(mv),
                Err(e) => return Err(format!("{e}")),
            }
        }
        Ok(b)
    }

    pub fn extend(&mut self, s: &str) -> Result<(), String> {
        println!("{s}");
        for m in s.split_whitespace() {
            match move_from_str(m) {
                Ok(mv) => self.mv(mv),
                Err(e) => return Err(format!("{e}")),
            }
        }
        Ok(())
    }
}