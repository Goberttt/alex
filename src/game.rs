use crate::Board;
use crate::enums::{ Notation, Move };
use crate::helpers::string_from_move;
use std::io;
use std::io::Write;

use rayon::prelude::*;

#[derive(Clone)]
pub struct GameState {
	pub board: Board,
	pub children: Vec<usize>,
	pub parent: Option<usize>,
	pub mv_from_parent: Option<Move>,
	pub score: Option<isize>,
}


pub fn brute_force(mut game: GameState, depth: usize, notation: Notation) -> isize {
	let mut games = Vec::new();

	for mv in game.board.all_legal_moves() {
		games.push(
			GameState {
				board: game.board.mv_new_no_memory(&mv),
				children: vec![],
				parent: None,
				mv_from_parent: Some(mv.clone()),
				score: None,
			});
	}
	let mult = games.len();
	let results: Vec<(Vec<String>, isize)> = 
		games
			.par_iter().enumerate()
			.map(move |(i, game)| _brute_force(game.clone(), depth, notation.clone(), i==0, mult))
			.collect();

	let mut score = 0;
	match game.board.to_move {
		0 => score = *results.iter().map(|(_seq, score)| score).max().unwrap(),
		1 => score = *results.iter().map(|(_seq, score)| score).min().unwrap(),
		_ => (), //cant happen
	};

	print!("    Best sequence:    ");
	for (seq, sc) in results.iter().rev() {
		if *sc == score {
			for m in seq {
				print!("{} ", m);
			}
			break;
		}
	}
	println!("");

	score
}

fn _brute_force(game: GameState, depth: usize, notation: Notation, talk: bool, mult: usize) -> (Vec<String>, isize) {
	let mut arena = vec![vec![game]];

	for d in 1..=depth {
		if talk { print!("    Generating level {}: ", d+1) };
		io::stdout().flush().expect("flush failed!");
		let mut new = Vec::new();
		let mut j = 0;
		for (i,g) in arena[d-1].iter_mut().enumerate() {
			for mv in g.board.all_legal_moves().iter() {
				g.children.push(j);
				new.push(
					GameState {
						board: g.board.mv_new_no_memory(&mv),
						children: vec![],
						parent: Some(i),
						mv_from_parent: Some(mv.clone()),
						score: None,
					});
				j += 1;
			}
		}
		if talk { println!("Done! Estimate of total number of games in this level: {}", new.len()*mult) };
		arena.push(new);
	}
	if talk { println!("\n    Finding best sequence...") };
	minmax(&mut arena);

	let score = arena[0][0].score.unwrap();
	let mut level = 0;
	let mut current = 0;
	'outer: for _d in 1..=depth {
		for child in &arena[level][current].children {
			match arena[level+1][*child].score {
				None => (),
				Some(num) => if num == score {
					current = *child;
					level += 1;
					continue 'outer;
				}
			}
		}
	}
	//print!("    Best sequence:    ");
	let mut seq = Vec::new();
	seq.push(string_from_move(&arena[0][0].mv_from_parent.clone().unwrap(), arena[0][0].board.players[(arena[0][0].board.to_move+1)%2], notation));
	loop {
		let game = &arena[level][current];
		match game.parent {
			Some(p) => {
				seq.push(string_from_move(&game.mv_from_parent.clone().unwrap(), game.board.players[(game.board.to_move+1)%2], notation));
				current = p;
				level -= 1;
			},
			None => break,
		}
	}

	(seq, score)
}

impl GameState {
	fn score(&self) -> isize {
		if self.board.current_player_wins() {
			match self.board.to_move {
				0 => return 1000,
				1 => return -1000,
				_ => (), //cant happen
			};
		};

		let mut sum: isize = 0;
		sum += self.board.walls_left[0] as isize - self.board.walls_left[1] as isize;
		sum += 2*(self.board.dist_to_goal(1).unwrap() as isize - self.board.dist_to_goal(0).unwrap() as isize);
		if self.board.to_move == 0 { sum += 1 } else { sum -= 1 };
		sum
	}
}

fn minmax (arena: &mut Vec<Vec<GameState>>) {
	let mut current = 0;
	let mut level = 0;

	'outer: loop {
		if arena[level][current].children.is_empty() || arena[level][current].board.current_player_wins() {
			arena[level][current].score = Some(arena[level][current].score());
			match arena[level][current].parent {
				Some(p) => {
					current = p; level -= 1; continue 'outer;
				},
				None => return,
			}
		}
		for child in &arena[level][current].children {
			if arena[level+1][*child].score.is_none() {
				level += 1;
				current = *child;
				continue 'outer;
			}
		}

		//all children have a score

		let mut scores: Vec<isize> = Vec::new();
		for child in &arena[level][current].children {
			scores.push(arena[level+1][*child].score.unwrap());
		}
		match arena[level][current].board.to_move {
			0 => arena[level][current].score = Some(*scores.iter().max().unwrap()),
			1 => arena[level][current].score = Some(*scores.iter().min().unwrap()),
			_ => (), //cant happen
		}
		match arena[level][current].parent {
			Some(p) => {
				current = p; level -= 1;
			},
			None => return,
		}
	}
}


/*
pub fn brute_force(game: GameState) -> bool {
	let mut current_player_wins = HashSet::new();
	let mut current_player_looses = HashSet::new();

	let mut arena = vec![game]; //contains all gamestates, so they all have the same lifetime (a nice trick in rust).
	//store the indices instead of references. They behave like raw pointers but are safe(ish)

	//add children of root
	let mut i = 0;
	for mv in arena[0].board.all_legal_moves().iter() {
		i += 1;
		arena[0].children.push(i);
		arena.push(
			GameState {
				board: arena[0].board.mv_new(&mv),
				children: vec![],
				parent: Some(0),
				is_winning: false,
				is_loosing: false,
			});
	}

	let mut current = 0; //"pointer" to the current gamestate we are lookíng at

	'outer: loop {
		if arena.len()%10000 == 0 { println!("{}", arena.len()) };
		//println!("{}", arena.len());
		if arena[current].board.current_player_wins() { //this position is winning
			arena[current].is_winning = true;
			current_player_wins.insert(arena[current].board.hash());
			match arena[current].parent {
				Some(p) => current = p,
				None => return true,
			}
			continue 'outer;
		}
		for next in arena[current].children.iter() {
			if arena[*next].is_loosing || current_player_looses.contains(&arena[*next].board.hash()) { //found a winning move for this position
				arena[current].is_winning = true;
				current_player_wins.insert(arena[current].board.hash());
				match arena[current].parent {
					Some(p) => current = p,
					None => return true,
				}
				continue 'outer;
			}
		}

		//there are no winning moves

		//goto undetermined child
		for next in arena[current].children.iter() {
			if !current_player_wins.contains(&arena[*next].board.hash())
				&& !current_player_looses.contains(&arena[*next].board.hash()) {
				current = *next;
				//add children
				i = arena.len()-1;
				for mv in arena[current].board.all_legal_moves().iter() {
					i += 1;
					arena[current].children.push(i);
					arena.push(
						GameState {
							board: arena[current].board.mv_new(&mv),
							children: vec![],
							parent: Some(current),
							is_winning: false,
							is_loosing: false,
						});
				}
				continue 'outer;
			}
		}

		//there are no undetermined moves, all moves are loosing

		arena[current].is_loosing = true;
		current_player_looses.insert(arena[current].board.hash());
		match arena[current].parent {
			Some(p) => current = p,
			None => return false,
		}
	}
}
*/