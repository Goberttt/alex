use std::collections::HashMap;

use crate::move_from_str;
use crate::enums::Move;
use crate::enums::Move::*;
use crate::graph::Graph;
use crate::enums::MoveError;
use crate::enums::MoveError::*;
use crate::enums::ParseError;
use crate::enums::ParseError::*;
use crate::enums::Notation;

#[derive(Clone)]
pub struct Board {
    pub walls: [[[bool; 8]; 8]; 2],
    pub players: [[usize; 2]; 2],
    pub to_move: usize,
    pub game_graph: Graph,
    pub walls_left: [usize; 2],
    pub move_sequence: Vec<Move>,
}


/*
impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.walls.hash(state);
        self.players.hash(state);
        self.to_move.hash(state);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.walls == other.walls && self.players == other.players && self.to_move == other.to_move
    }
}

impl Eq for Board {}

*/

impl Default for Board {
    fn default() -> Board {
        Board {
            walls: [[[false; 8]; 8]; 2],
            players: [[4,0],[4,8]],
            to_move: 0,
            game_graph: Default::default(),
            walls_left: [10, 10],
            move_sequence: vec![],
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {..Default::default()}
    }

    pub fn mv(&mut self, m: &Move) {
        //actually applies a move
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
            Wall([i, x, y]) => { self.walls[*i][*x][*y] = true; self.game_graph.place_wall([*i, *x, *y]); self.walls_left[self.to_move] -= 1; },
        }
        if self.to_move == 0 { self.to_move = 1 } else { self.to_move = 0 };
    }

    pub fn mv_new(&self, m: &Move) -> Board {
        //returns a new board with the given move applies. Check if move is legal before calling!
        let mut b = self.clone();
        b.mv(m);
        return b;
    }

    pub fn from(s: &str, move_errors: &HashMap<MoveError, String>, notation: Notation) -> Result<Board, String> {
        let mut b = Board::new();
        match b.extend(s, move_errors, notation) {
            Ok(()) => return Ok(b),
            Err(e) => return Err(e),
        }
    }

    fn check_move(&mut self, mv: &Move) -> Result<(), MoveError> {
        //checks if a move is illegal. Horrible case distinction
        //to_move is true if its player 0's turn, else its false. This is to check a move sequence ocrrectly
        let mut tm = 0; //to move
        let mut ntm = 0; //not to move
        match self.to_move {
            0 => { tm = 0; ntm = 1 },
            1 => { tm = 1; ntm = 0 },
            _ => () //cannot happen
        };
        let [mut x, mut y] = self.players[tm];
        let [a,b] = self.players[ntm];
        match mv {
            //first checking single step
            Step(1) => {
                if y == 8 { return Err(EdgeOfBoard) };
                if (x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y]) { return Err(BlockedByWall) };
                if [a, b] == [x, y+1] {return Err(BlockedByOpponent) };
            },
            Step(2) => {
                if x == 8 { return Err(EdgeOfBoard) };
                if (y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1]) { return Err(BlockedByWall) };
                if [a, b] == [x+1, y] {return Err(BlockedByOpponent) };
            },
            Step(3) => {
                if y == 0 { return Err(EdgeOfBoard) };
                if (x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1]) { return Err(BlockedByWall) };
                if [a, b] == [x, y-1] {return Err(BlockedByOpponent) };
            },
            Step(4) => {
                if x == 0 { return Err(EdgeOfBoard) };
                if (y != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1]) { return Err(BlockedByWall) };
                if [a, b] == [x-1, y] {return Err(BlockedByOpponent) };
            },
            //checking jumps
            Step(mut d) => {
                let mut code: Vec<usize> = vec![];
                while {
                    code.push(d % 10);
                    d /= 10;
                    d != 0
                } {};
                
                //check if the jump is blocked by a wall
                match code[1] {
                    1 => {
                        if y == 8 { return Err(EdgeOfBoard) };
                        if (x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y]) { return Err(BlockedByWall) };
                    },
                    2 => {
                        if x == 8 { return Err(EdgeOfBoard) };
                        if (y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1]) { return Err(BlockedByWall) };
                    },
                    3 => {
                        if y == 0 { return Err(EdgeOfBoard) };
                        if (x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1]) { return Err(BlockedByWall) };
                    },
                    4 => {
                        if x == 0 { return Err(EdgeOfBoard) };
                        if (y != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1]) { return Err(BlockedByWall) };
                    },
                    _ => () //cannot happen
                };

                //opponent needs to be in that direction, if yes, move to tile of opponent
                match code[1] {
                    1 => if b == 0 || [x,y] != [a,b-1] { return Err(OpponentNotThere); } else { y += 1 },
                    2 => if a == 0 || [x,y] != [a-1,b] { return Err(OpponentNotThere); } else { x += 1 },
                    3 => if [x,y] != [a,b+1] { return Err(OpponentNotThere); } else { y -= 1 },
                    4 => if [x,y] != [a+1,b] { return Err(OpponentNotThere); } else { x -= 1 },
                    _ => (), // cannot happen
                };
                
                //jump in a straight line, need no wall or edge of board behind opponent
                if code[0] == code[1] {
                    match code[0] {
                        1 => {
                            if y == 8 { return Err(EdgeOfBoard) };
                            if (x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y]) { return Err(BlockedByWall) };
                        },
                        2 => {
                            if x == 8 { return Err(EdgeOfBoard) };
                            if (y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1]) { return Err(BlockedByWall) };
                        },
                        3 => {
                            if y == 0 { return Err(EdgeOfBoard) };
                            if (x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1]) { return Err(BlockedByWall) };
                        },
                        4 => {
                            if x == 0 { return Err(EdgeOfBoard) };
                            if (y != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1]) { return Err(BlockedByWall) };
                        },
                        _ => (), //cannot happen
                    }
                } else {
                    //jump diagonal, need wall or edge of board behind
                    match code[1] {
                        1 => if !((y == 8 || x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y])) { return Err(SpaceBehindFree) },
                        2 => if !((x == 8 || y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1])) { return Err(SpaceBehindFree) },
                        3 => if !((y == 0 || x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1])) { return Err(SpaceBehindFree) },
                        4 => if !((x == 0 || y != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1])) { return Err(SpaceBehindFree) },
                        _ => (), //cannot happen
                    };
                    //check if the next step is valid
                    match code[0] {
                        1 => {
                            if y == 8 { return Err(EdgeOfBoard) };
                            if (x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y]) { return Err(BlockedByWall) };
                        },
                        2 => {
                            if x == 8 { return Err(EdgeOfBoard) };
                            if (y != 8 && self.walls[1][x][y]) || (y != 0 && self.walls[1][x][y-1]) { return Err(BlockedByWall) };
                        },
                        3 => {
                            if y == 0 { return Err(EdgeOfBoard) };
                            if (x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1]) { return Err(BlockedByWall) };
                        },
                        4 => {
                            if x == 0 { return Err(EdgeOfBoard) };
                            if (y != 8 && self.walls[1][x-1][y]) || (y != 0 && self.walls[1][x-1][y-1]) { return Err(BlockedByWall) };
                        },
                        _ => (), //cannot happen
                    }
                }
            }
            Wall([i, x, y]) => { 
                if self.walls_left[self.to_move] == 0 { return Err(NoWallsLeft) };
                if self.walls[0][*x][*y] || self.walls[1][*x][*y] { return Err(SpaceOccupied) };
                match i {
                    0 => if (*x != 0 && self.walls[0][*x-1][*y]) || (*x != 7 && self.walls[0][*x+1][*y]) 
                    { return Err(SpaceOccupied) },
                    1 => if (*y != 0 && self.walls[1][*x][*y-1]) || (*y != 7 && self.walls[1][*x][*y+1]) 
                    { return Err(SpaceOccupied) },
                    _ => () //cannot happen
                };
                self.game_graph.place_wall([*i,*x,*y]);
                if self.game_graph.dist_to_goal(self.players[0], 8).is_none() {
                    self.game_graph.remove_wall([*i,*x,*y]);
                    return Err(P1NoPath);
                }
                if self.game_graph.dist_to_goal(self.players[1], 0).is_none() {
                    self.game_graph.remove_wall([*i,*x,*y]);
                    return Err(P2NoPath);
                }
                self.game_graph.remove_wall([*i,*x,*y]);
            }
        }
        Ok(())
    }

    fn check_step_to_goal(&self) -> Result<(),()> {
        let [x, y] = self.players[self.to_move];
        if self.to_move == 0 && ((x != 8 && self.walls[0][x][y]) || (x != 0 && self.walls[0][x-1][y])) { return Err(()) };
        if self.to_move == 1 && ((x != 8 && self.walls[0][x][y-1]) || (x != 0 && self.walls[0][x-1][y-1])) { return Err(()) };
        Ok(())
    }

    pub fn extend(&mut self, s: &str, move_errors: &HashMap<MoveError, String>, notation: Notation) -> Result<(), String> {
        //extends the board by the move sequence, returns an error if a move is illegal
        let old_board = self.clone();
        let mut tm = 0; //to move
        let mut ntm = 0; //not to move
        match self.to_move {
            0 => { tm = 0; ntm = 1 },
            1 => { tm = 1; ntm = 0 },
            _ => () //cannot happen
        };
        
        for (i,m) in s.split_whitespace().enumerate() {
            match move_from_str(m, self.players[tm], self.players[ntm], notation) {
                Ok(mv) => {
                    match self.check_move(&mv) {
                    Ok(()) => (),
                    Err(err) => { 
                        *self = old_board;
                        return Err(format!("Move {} ( {} ) is illegal, because: {}.\n    The board was not changed.",
                            i+1, m, move_errors.get(&err).unwrap())) },
                    };
                    self.mv(&mv);
                }
                Err(_) => { *self = old_board; return Err(format!("Input {} ( {} ) is invalid encoding.\n    The board was not changed.",
                    i+1, m)) },
            };
        }
        Ok(())
    }

    pub fn extend_no_check(&mut self, s: &str, notation: Notation) {
        //extends the board by the move sequences, might panic if a move is illegal
        let mut tm = 0; //to move
        let mut ntm = 0; //not to move
        match self.to_move {
            0 => { tm = 0; ntm = 1 },
            1 => { tm = 1; ntm = 0 },
            _ => () //cannot happen
        };
        for m in s.split_whitespace() {
            match move_from_str(m, self.players[tm], self.players[ntm], notation) {
                Ok(mv) => self.mv(&mv),
                _ => panic!("Not a move"),
            }
        }
    }

    pub fn all_legal_moves(&mut self) -> Vec<Move> {
        //returns all legal moves

        let mut moves = Vec::new();
        //orders the legal move such that in the brute force dfs we walk towards the goal greedely. Dirty implementation, fix later
        if self.to_move == 0 {
            for dir in [1,11,12,14,21,2,4,22,23,41,44,43,3,32,33,34] {
                if self.check_move(&Step(dir)).is_ok() { moves.push(Step(dir)) };
            } 
        } else {
            for dir in [3,33,32,34,2,4,23,22,21,44,43,41,1,14,12,11] {
                if self.check_move(&Step(dir)).is_ok() { moves.push(Step(dir)) };
            } 
        }
        for i in 0..=1 {
            for x in 0..=7 {
                for y in 0..=7 {
                    if self.check_move(&Wall([i, x, y])).is_ok() { moves.push(Wall([i, x, y])) };
                }
            }
        }
        moves
    }

    pub fn fill(&mut self, [x_0,y_0]: [usize; 2], [x_1,y_1]: [usize; 2]) {
        for i in 0..=1 {
            for x in x_0..=x_1 {
                for y in y_0..=y_1 {
                    self.walls[i][x][y] = true;
                    self.game_graph.place_wall([i,x,y]);
                }
            }
        }
    }

    pub fn try_fill_from_str(&mut self, input: String) -> Result<(), ParseError> {
        if let [x_0,y_0,x_1,y_1] = input.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>()[..] {
            self.fill([x_0,y_0], [x_1,y_1]);
        } else {
            return Err(FillChordsIncorrect)
        }
        Ok(())
    }

    pub fn current_player_wins(& self) -> bool {
        match self.to_move {
            0 => return self.players[0][1] == 7 && self.check_step_to_goal().is_ok() && self.players[1][1] != 0,
            1 => return self.players[1][1] == 1 && self.check_step_to_goal().is_ok() && self.players[0][1] != 8,
            _ => return false //cannot happen
        }
    }

    pub fn hash(&self) -> [u64; 3] {
        let mut horizontal = 0;
        let mut vertical = 0;
        let mut positions_and_to_move = 0;

        //horizontan:
        for x in 0..=7 {
            for y in 0..=7 {
                if self.walls[0][x][y] { horizontal += 1 };
                horizontal = horizontal << 1;
            }
        }
        //vertical:
        for x in 0..=7 {
            for y in 0..=7 {
                if self.walls[1][x][y] { vertical += 1 };
                vertical = vertical << 1;
            }
        }

        //player0:
        positions_and_to_move += self.players[0][0];
        positions_and_to_move = positions_and_to_move << 4;
        positions_and_to_move += self.players[0][1];
        positions_and_to_move = positions_and_to_move << 4;
        //player1:
        positions_and_to_move += self.players[1][0];
        positions_and_to_move = positions_and_to_move << 4;
        positions_and_to_move += self.players[1][1];
        positions_and_to_move = positions_and_to_move << 4;

        //to_move:
        positions_and_to_move += self.to_move;

        [horizontal, vertical, positions_and_to_move.try_into().unwrap()]
    }
}


//m Va1 Va3 Va5 Va7 Vb1 Vb3 Vb5 Vb7 Vc1 Vc3 Vc5 Vc7 Vd1 Vd3 Vd5 Vd7 Ve1 Ve3 Ve5 Ve7 Vf1 Vf3 Vf5 Vf7 Vg1 Vg3 Vg5 Vg7 Vh1 Vh3 Vh5 Vh7 Ha8 Hc8 Hf8 Hh8 Ha6 Hc6 Hf6 Hh6 Ha4 Hc4 Hf4 Hh4 Ha2 Hc2 Hf2 Hh2
//m E E E E E E Vg1 Vg3 Vg5 Vg7 Hf1 Hf2 Hf3 Hf4 Hf5 Hf6 Hf7 Hf8 Hd1 Hd2 Hd3 Hd4 Hd5 Hd6 Hd7 Hd8 Hb1 Hb2 Hb3 Hb4 Hb5 Hb6 Hb7 Hb8 Va8 Va6 Va4 Va2