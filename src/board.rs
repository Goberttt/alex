use crate::move_from_str;
use crate::enums::Move;
use crate::enums::Move::*;
use crate::graph::Graph;

pub struct Board {
    pub walls: [[[bool; 8]; 8]; 2],
    pub players: [[usize; 2]; 2],
    pub to_move: usize,
    pub game_graph: Graph,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            walls: [[[false; 8]; 8]; 2],
            players: [[4,0],[4,8]],
            to_move: 0,
            game_graph: Default::default(),
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {..Default::default()}
    }
    pub fn mv(&mut self, m: &Move){
        //detect illegal moves before calling! Might panic if illegal move is passed
        match m {
            Step(mut d) => {
                let mut code: Vec<usize> = vec![];
                while {
                    code.push(d % 10);
                    d /= 10;
                    d != 0
                } {};
                for i in code.iter().rev() {
                    match i {
                    1 => self.players[self.to_move][1] += 1, //N
                    2 => self.players[self.to_move][0] += 1, //E
                    3 => self.players[self.to_move][1] -= 1, //S
                    4 => self.players[self.to_move][0] -= 1, //W
                    _ => (), //todo illegal input
                    }

                }
            },
            Wall([i, x, y]) => { self.walls[*i][*x][*y] = true; self.game_graph.place_wall([*i, *x, *y]); },
        }
        if self.to_move == 0 {self.to_move = 1} else {self.to_move = 0};
    }

    pub fn from(s: &str) -> Result<Board, String> {
        let mut b = Board::new();
        match b.extend(s) {
            Ok(()) => return Ok(b),
            Err(s) => return Err(s),
        }
    }

    fn check_move(&mut self, mv: &Move, to_move: bool) -> Result<(), String> {
        let tm; //to move
        let ntm; //not to move
        match to_move {
            true => { tm = 0; ntm = 1 },
            false => { tm = 1; ntm = 0 },
        };
        let [mut x, mut y] = self.players[tm];
        let [a,b] = self.players[ntm];
        match mv {
            //first checking single step
            Step(1) => {
                if y == 8 { return Err(format!("Cannot move up, edge of board")) };
                if (x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y]) { return Err(format!("Cannot move up, blocked by wall")) };
                if [a, b] == [x, y+1] {return Err(format!("Cannot move up, space occupied by opponent.")) };
            },
            Step(2) => {
                if x == 8 { return Err(format!("Cannot move right, edge of board")) };
                if (y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1]) { return Err(format!("Cannot move right, blocked by wall")) };
                if [a, b] == [x+1, y] {return Err(format!("Cannot move right, space occupied by opponent.")) };
            },
            Step(3) => {
                if y == 0 { return Err(format!("Cannot move down, edge of board")) };
                if (x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1]) { return Err(format!("Cannot move down, blocked by wall")) };
                if [a, b] == [x, y-1] {return Err(format!("Cannot move down, space occupied by opponent.")) };
            },
            Step(4) => {
                if x == 0 { return Err(format!("Cannot move left, edge of board")) };
                if (y != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1]) { return Err(format!("Cannot move left, blocked by wall")) };
                if [a, b] == [x-1, y] {return Err(format!("Cannot move left, space occupied by opponent.")) };
            },
            //checking jumps
            Step(mut d) => {
                let mut code: Vec<usize> = vec![];
                while {
                    code.push(d % 10);
                    d /= 10;
                    d != 0
                } {};
                //opponent needs to be in that direction, if yes, move to tile of opponent
                match code[1] {
                    1 => if y != b-1 { return Err(format!("Cannot jump by moving up, opponent is not there")); } else { y += 1 },
                    2 => if x != a-1 { return Err(format!("Cannot jump by moving right, opponent not there")); } else { x += 1 },
                    3 => if y != b+1 { return Err(format!("Cannot jump by moving down, opponent not there")); } else { y -= 1 },
                    4 => if x != a+1 { return Err(format!("Cannot jump by moving left, opponent not there")); } else { x -= 1 },
                    _ => (), // cannot happen
                };
                //jump in a straight line, need no wall or edge of board behind opponent
                if code[0] == code[1] {
                    match code[0] {
                        1 => {
                            if y == 8 { return Err(format!("Cannot jump up, edge of board")) };
                            if (x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y]) { return Err(format!("Cannot jump up, blocked by wall")) };
                        },
                        2 => {
                            if x == 8 { return Err(format!("Cannot jump right, edge of board")) };
                            if (y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1]) { return Err(format!("Cannot jump right, blocked by wall")) };
                        },
                        3 => {
                            if y == 0 { return Err(format!("Cannot jump down, edge of board")) };
                            if (x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1]) { return Err(format!("Cannot jump down, blocked by wall")) };
                        },
                        4 => {
                            if x == 0 { return Err(format!("Cannot jump left, edge of board")) };
                            if (x != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1]) { return Err(format!("Cannot jump left, blocked by wall")) };
                        },
                        _ => (), //cannot happen
                    }
                } else {
                    //jump diagonal, need wall or edge of board behind
                    match code[1] {
                        1 => if !((x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y]) || y == 8) { return Err(format!("Cannot jump diagonal, space behind is free")) },
                        2 => if !((y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1]) || x == 8) { return Err(format!("Cannot jump diagonal, space behind is free")) },
                        3 => if !((x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1]) || y == 0) { return Err(format!("Cannot jump diagonal, space behind is free")) },
                        4 => if !((y != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1]) || x == 0) { return Err(format!("Cannot jump diagonal, space behind is free")) },
                        _ => (), //cannot happen
                    };
                    //check if the next step is valid
                    match code[0] {
                        1 => {
                            if y == 8 { return Err(format!("Cannot jump up, edge of board")) };
                            if (x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y]) { return Err(format!("Cannot jump up, blocked by wall")) };
                        },
                        2 => {
                            if x == 8 { return Err(format!("Cannot jump right, edge of board")) };
                            if (y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1]) { return Err(format!("Cannot jump right, blocked by wall")) };
                        },
                        3 => {
                            if y == 0 { return Err(format!("Cannot jump down, edge of board")) };
                            if (x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1]) { return Err(format!("Cannot jump down, blocked by wall")) };
                        },
                        4 => {
                            if x == 0 { return Err(format!("Cannot jump left, edge of board")) };
                            if (y != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1]) { return Err(format!("Cannot jump left, blocked by wall")) };
                        },
                        _ => (), //cannot happen
                    }
                }
            }
            Wall([i, x, y]) => { 
                if self.walls[0][*x][*y] || self.walls[1][*x][*y] { return Err(format!("Cannot place wall, space already occupied")) };
                match i {
                    0 => if (*x != 0 && self.walls[0][*x-1][*y]) || (*x != 7 && self.walls[0][*x+1][*y]) { return Err(format!("Cannot place wall, space already occupied")) },
                    1 => if (*y != 0 && self.walls[1][*x][*y-1]) || (*y != 7 && self.walls[1][*x][*y+1]) { return Err(format!("Cannot place wall, space already occupied")) },
                    _ => () //cannot happen
                };
                if self.game_graph.dist_to_goal([i, x, y], self.players[0], 8) == None { return Err(format!("Cannot place wall, player 1 has no path to the goal")) };
                if self.game_graph.dist_to_goal([i, x, y], self.players[1], 0) == None { return Err(format!("Cannot place wall, player 2 has no path to the goal")) };
                }
        }
        Ok(())
    }

    pub fn extend(&mut self, s: &str) -> Result<(), String> {
        let mut moves = Vec::new();
        let mut tm;
        match self.to_move {
            0 => tm = true,
            1 => tm = false,
            _ => tm = true, //cant happen
        };
        for (i,m) in s.split_whitespace().enumerate() {
            match move_from_str(m) {
                Ok(mv) => {
                    match self.check_move(&mv, tm) {
                    Ok(()) => (),
                    Err(s) => return Err(format!("Move {} ( {} ) is illegal, because: {}.\n    The board was not changed.", i+1, m, s)),
                    };
                    moves.push(mv);
                }
                Err(e) => return Err(format!("{e}")),
            };
            tm = !tm;
        }

        for mv in moves.iter() {
            self.mv(mv);
        }
        Ok(())
    }
}