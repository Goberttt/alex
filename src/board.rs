use std::collections::{ HashMap, HashSet };

use crate::enums::{ Move, Move::*,
    MoveError, MoveError::*,
    ParseError, ParseError::*,
    Notation,
    Player, Player::* };

use crate::move_from_str;
//use crate::graph::Graph;

#[derive(Clone)]
pub struct Board {
    pub walls: u128,               //bitmap. First 64 horizontal, other 64 vertival, 64*i + 8*y + x is bit for wall [i, x, y]
    pub players: [[usize; 2]; 2],
    pub to_move: Player,
    pub walls_left: [usize; 2],
    pub move_sequence: Vec<Move>,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            walls: 0, 
            players: [[4,0],[4,8]],
            to_move: Player1,
            walls_left: [10, 10],
            move_sequence: vec![],
        }
    }
}

impl Board {
    fn is_wall(&self, i: usize, x: usize, y: usize) -> bool {
        self.walls & (1 << (64*i + 8*y + x)) != 0
    }

    fn place_wall(&mut self, i: usize, x: usize, y: usize) {
        self.walls = self.walls | (1 << (64*i + 8*y + x));
    }

    fn remove_wall(&mut self, i: usize, x: usize, y: usize) {
        self.walls = self.walls & !(1 << (64*i + 8*y + x));
    }

    pub fn to_move_indices(&self) -> (usize, usize) {
        match self.to_move {
            Player1 => (0, 1),
            Player2 => (1, 0)
        }
    }

    pub fn new() -> Board {
        Board {..Default::default()}
    }

    pub fn new5x5() -> Board {
        Board {
            walls: 80770148214459885546621745580772163713, 
            players: [[4,2],[4,6]],
            to_move: Player1,
            walls_left: [3, 3],
            move_sequence: vec![],
        }
    }

    pub fn mv(&mut self, m: &Move) {
        //actually applies a move
        //detect illegal moves before calling! Might panic if illegal move is passed
        let tm = self.to_move_indices().0;
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
                    1 => self.players[tm][1] += 1, //N
                    2 => self.players[tm][0] += 1, //E
                    3 => self.players[tm][1] -= 1, //S
                    4 => self.players[tm][0] -= 1, //W
                    _ => (),
                    }

                }
            },
            Wall([i, x, y]) => { 
                self.place_wall(*i,*x,*y);
                self.walls_left[tm] -= 1;
            },
        }
        match self.to_move {
            Player1 => self.to_move = Player2,
            Player2 => self.to_move = Player1
        }
        self.move_sequence.push(m.clone());
    }

    pub fn mv_new_no_memory(&self, m: &Move) -> Board {
        //makes the move but doesnt save it to memory
        let mut b = self.mv_new(m);
        b.move_sequence.clear();
        b
    }

    pub fn mv_new(&self, m: &Move) -> Board {
        //returns a new board with the given move applies. Check if move is legal before calling!
        let mut b = self.clone();
        b.mv(m);
        b
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

        let (tm, ntm) = self.to_move_indices();
        let [mut x, mut y] = self.players[tm];
        let [a,b] = self.players[ntm];
        match mv {
            //first checking single step
            Step(1) => {
                if y == 8 { return Err(EdgeOfBoard) };
                if (x != 8 && self.is_wall(0, x, y)) || (x != 0 && self.is_wall(0, x-1, y)) { return Err(BlockedByWall) };
                if [a, b] == [x, y+1] {return Err(BlockedByOpponent) };
            },
            Step(2) => {
                if x == 8 { return Err(EdgeOfBoard) };
                if (y != 8 && self.is_wall(1, x, y)) || (y != 0 && self.is_wall(1, x, y-1)) { return Err(BlockedByWall) };
                if [a, b] == [x+1, y] {return Err(BlockedByOpponent) };
            },
            Step(3) => {
                if y == 0 { return Err(EdgeOfBoard) };
                if (x != 8 && self.is_wall(0, x, y-1)) || (x != 0 && self.is_wall(0, x-1, y-1)) { return Err(BlockedByWall) };
                if [a, b] == [x, y-1] {return Err(BlockedByOpponent) };
            },
            Step(4) => {
                if x == 0 { return Err(EdgeOfBoard) };
                if (y != 8 && self.is_wall(1, x-1, y)) || (y != 0 && self.is_wall(1, x-1, y-1)) { return Err(BlockedByWall) };
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
                        if (x != 8 && self.is_wall(0, x, y)) || (x != 0 && self.is_wall(0, x-1, y)) { return Err(BlockedByWall) };
                    },
                    2 => {
                        if x == 8 { return Err(EdgeOfBoard) };
                        if (y != 8 && self.is_wall(1, x, y)) || (y != 0 && self.is_wall(1, x, y-1)) { return Err(BlockedByWall) };
                    },
                    3 => {
                        if y == 0 { return Err(EdgeOfBoard) };
                        if (x != 8 && self.is_wall(0, x, y-1)) || (x != 0 && self.is_wall(0, x-1, y-1)) { return Err(BlockedByWall) };
                    },
                    4 => {
                        if x == 0 { return Err(EdgeOfBoard) };
                        if (y != 8 && self.is_wall(1, x-1, y)) || (y != 0 && self.is_wall(1, x-1, y-1)) { return Err(BlockedByWall) };
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
                            if (x != 8 && self.is_wall(0, x, y)) || (x != 0 && self.is_wall(0, x-1, y)) { return Err(BlockedByWall) };
                        },
                        2 => {
                            if x == 8 { return Err(EdgeOfBoard) };
                            if (y != 8 && self.is_wall(1, x, y)) || (y != 0 && self.is_wall(1, x, y-1)) { return Err(BlockedByWall) };
                        },
                        3 => {
                            if y == 0 { return Err(EdgeOfBoard) };
                            if (x != 8 && self.is_wall(0, x, y-1)) || (x != 0 && self.is_wall(0, x-1, y-1)) { return Err(BlockedByWall) };
                        },
                        4 => {
                            if x == 0 { return Err(EdgeOfBoard) };
                            if (y != 8 && self.is_wall(1, x-1, y)) || (y != 0 && self.is_wall(1, x-1, y-1)) { return Err(BlockedByWall) };
                        },
                        _ => (), //cannot happen
                    }
                } else {
                    //jump diagonal, need wall or edge of board behind
                    match code[1] {
                        1 => if !((y == 8 || x != 8 && self.is_wall(0, x, y)) || (x != 0 && self.is_wall(0, x-1, y))) { return Err(SpaceBehindFree) },
                        2 => if !((x == 8 || y != 8 && self.is_wall(1, x, y)) || (y != 0 && self.is_wall(1, x, y-1))) { return Err(SpaceBehindFree) },
                        3 => if !((y == 0 || x != 8 && self.is_wall(0, x, y-1)) || (x != 0 && self.is_wall(0, x-1, y-1))) { return Err(SpaceBehindFree) },
                        4 => if !((x == 0 || y != 8 && self.is_wall(1, x-1, y)) || (y != 0 && self.is_wall(1, x-1, y-1))) { return Err(SpaceBehindFree) },
                        _ => (), //cannot happen
                    };
                    //check if the next step is valid
                    match code[0] {
                        1 => {
                            if y == 8 { return Err(EdgeOfBoard) };
                            if (x != 8 && self.is_wall(0, x, y)) || (x != 0 && self.is_wall(0, x-1, y)) { return Err(BlockedByWall) };
                        },
                        2 => {
                            if x == 8 { return Err(EdgeOfBoard) };
                            if (y != 8 && self.is_wall(1, x, y)) || (y != 0 && self.is_wall(1, x, y-1)) { return Err(BlockedByWall) };
                        },
                        3 => {
                            if y == 0 { return Err(EdgeOfBoard) };
                            if (x != 8 && self.is_wall(0, x, y-1)) || (x != 0 && self.is_wall(0, x-1, y-1)) { return Err(BlockedByWall) };
                        },
                        4 => {
                            if x == 0 { return Err(EdgeOfBoard) };
                            if (y != 8 && self.is_wall(1, x-1, y)) || (y != 0 && self.is_wall(1, x-1, y-1)) { return Err(BlockedByWall) };
                        },
                        _ => (), //cannot happen
                    }
                }
            }
            Wall([i, x, y]) => { 
                if self.walls_left[tm] == 0 { return Err(NoWallsLeft) };
                if self.is_wall(0, *x, *y) || self.is_wall(1, *x, *y) { return Err(SpaceOccupied) };
                match i {
                    0 => if (*x != 0 && self.is_wall(0, *x-1, *y)) || (*x != 7 && self.is_wall(0, *x+1, *y)) 
                    { return Err(SpaceOccupied) },
                    1 => if (*y != 0 && self.is_wall(1, *x, *y-1)) || (*y != 7 && self.is_wall(1, *x, *y+1)) 
                    { return Err(SpaceOccupied) },
                    _ => () //cannot happen
                };
                self.place_wall(*i, *x, *y);
                if self.dist_to_goal(0).is_none() {
                    self.remove_wall(*i, *x, *y);
                    return Err(P1NoPath);
                }
                if self.dist_to_goal(1).is_none() {
                    self.remove_wall(*i, *x, *y);
                    return Err(P2NoPath);
                }
                self.remove_wall(*i, *x, *y);
            }
        }
        Ok(())
    }

    pub fn extend(&mut self, s: &str, move_errors: &HashMap<MoveError, String>, notation: Notation) -> Result<(), String> {
        //extends the board by the move sequence, returns an error if a move is illegal
        let old_board = self.clone();
        let (tm, ntm) = self.to_move_indices();
        
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
        let (tm, ntm) = self.to_move_indices();

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
        match self.to_move {
            Player1 => {
                for dir in [1,11,12,14,21,2,4,22,23,41,44,43,3,32,33,34] {
                    if self.check_move(&Step(dir)).is_ok() { moves.push(Step(dir)) };
                }
            },
            Player2 => {
                for dir in [3,33,32,34,2,4,23,22,21,44,43,41,1,14,12,11] {
                    if self.check_move(&Step(dir)).is_ok() { moves.push(Step(dir)) };
                }
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
            for x in x_0+1..x_1 {
                for y in y_0+1..y_1 {
                    self.place_wall(i, x, y);
                }
            }
            for x in x_0+1..x_1 {
                for y in [y_0, y_1] {
                    self.place_wall(0, x, y);
                }
            }
            for y in y_0+1..y_1 {
                for x in [x_0, x_1] {
                    self.place_wall(1, x, y);
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

    fn check_step_to_goal(&self) -> bool {
        let tm;
        match self.to_move {
            Player1 => tm = 0,
            Player2 => tm = 1,
        }
        let [x, y] = self.players[tm];
        match self.to_move {
            Player1 => !((x != 8 && y != 8 && self.is_wall(0, x, y)) || (x != 0 && y != 8 && self.is_wall(0, x-1, y))),
            Player2 => !((x != 8 && y != 0 && self.is_wall(0, x, y-1)) || (x != 0 && y != 0 && self.is_wall(0, x-1, y-1)))
        }
    }

    pub fn current_player_wins(&self) -> bool {
        match self.to_move {
            Player1 => self.players[0][1] >= 7 && self.check_step_to_goal() && self.players[1][1] != 0,
            Player2 => self.players[1][1] <= 1 && self.check_step_to_goal() && self.players[0][1] != 8,
        }
    }

    pub fn current_player_wins_5x5(&self) -> bool {
        match self.to_move {
            Player1 => self.players[0][1] >= 5 && self.check_step_to_goal() && self.players[1][1] != 0,
            Player2 => self.players[1][1] <= 3 && self.check_step_to_goal() && self.players[0][1] != 8,
        }
    }

    fn step_possible(&self, [x, y]: [usize; 2], dir: usize) -> bool {
        //returns if the step in dir from player is possible. dir is 1,2,3,4 for NESW
        //ignores the players
        match dir {
            1 => {
                if y == 8 { return false };
                if (x != 8 && self.is_wall(0, x, y)) || (x != 0 && self.is_wall(0, x-1, y)) { return false };
            },
            2 => {
                if x == 8 { return false };
                if (y != 8 && self.is_wall(1, x, y)) || (y != 0 && self.is_wall(1, x, y-1)) { return false };
            },
            3 => {
                if y == 0 { return false };
                if (x != 8 && self.is_wall(0, x, y-1)) || (x != 0 && self.is_wall(0, x-1, y-1)) { return false };
            },
            4 => {
                if x == 0 { return false };
                if (y != 8 && self.is_wall(1, x-1, y)) || (y != 0 && self.is_wall(1, x-1, y-1)) { return false };
            },
            _ => (), //cant happen
        }
        true
    }

    pub fn dist_to_goal(&self, p: usize) -> Option<usize> {
        //p is the player to be evaluated
        //panics if p is not 0 or 1
        let [x, y] = self.players[p];
        let g;
        match p {
            0 => g = 8,
            1 => g = 0,
            _ => g = 0, //cant happen
        }
        if y == g { return Some(0) };
        let mut front = vec![[x, y]];
        let mut found = HashSet::from([[x, y]]);
        let mut steps = 0;
        loop {
            steps += 1;
            let mut new_front = Vec::new();
            for [x, y] in front {
                //look up
                if self.step_possible([x, y], 1) && !found.contains(&[x, y+1]) {
                    if y+1 == g { return Some(steps) };
                    found.insert([x, y+1]);
                    new_front.push([x, y+1]);
                };
                //look right
                if self.step_possible([x, y], 2) && !found.contains(&[x+1, y]) {
                    found.insert([x+1, y]);
                    new_front.push([x+1, y]);
                };//look down
                if self.step_possible([x, y], 3) && !found.contains(&[x, y-1]) {
                    if y-1 == g { return Some(steps) };
                    found.insert([x, y-1]);
                    new_front.push([x, y-1]);
                };//look left
                if self.step_possible([x, y], 4) && !found.contains(&[x-1, y]) {
                    found.insert([x-1, y]);
                    new_front.push([x-1, y]);
                };
            }
            front = new_front;
            if front.is_empty() { return None };
        }
    }
}