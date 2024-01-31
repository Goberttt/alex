use crate::Board;
use crate::enums::{ Notation, Move, Move::* };
use crate::helpers::string_from_move;

use std::time::SystemTime;

use rayon::prelude::*;

#[derive(Clone)]
pub struct GameState {
	pub board: Board,
	//pub children: Vec<usize>,
	//pub parent: Option<usize>,
	pub mv_from_parent: Option<Move>,
	//pub score: Option<isize>,
}

pub fn brute_force(mut game: GameState, max_depth: usize, notation: Notation) -> isize {
	let begin_time = SystemTime::now();

	let results: Vec<_> = 
		game.board
			.all_legal_moves()
			.iter()
			.map(|mv| 
				GameState {
					board: game.board.mv_new_no_memory(mv),
					mv_from_parent: Some(mv.clone()),
				})
			.collect::<Vec<_>>()
			.par_iter()
			.map(move |game| _brute_force_recursive_dfs(game.clone(), 1, max_depth))
			.collect();

	let mut score = 0;
	match game.board.to_move {
		0 => score = *results.iter().map(|(_seq, score)| score).max().unwrap(),
		1 => score = *results.iter().map(|(_seq, score)| score).min().unwrap(),
		_ => (), //cant happen
	};

	let total_time = SystemTime::now().duration_since(begin_time).unwrap().as_secs();
	println!("    That took about {} seconds.\n", total_time);

	print!("    Best sequence found:    ");
	for (seq, sc) in results.iter() {
		if *sc == score {
			for m in seq.iter().rev() {
				print!("{} ", string_from_move(&m, game.board.players[game.board.to_move], notation));
			}
			break;
		}
	}
	println!("");

	score
}

fn _brute_force_recursive_dfs(
mut game: GameState,
depth: usize,
max_depth: usize) -> (Vec<Move>, isize) {

	if depth == max_depth || game.board.current_player_wins() {
		return (vec![game.mv_from_parent.clone().unwrap()], game.score());
	}

	let results: Vec<_> =
		game.board
			.all_legal_moves()
			.iter()
			.map(|mv| 
				GameState {
					board: game.board.mv_new_no_memory(mv),
					mv_from_parent: Some(mv.clone()),
				})
			.collect::<Vec<_>>()
			.par_iter()
			.map(move |game| _brute_force_recursive_dfs(game.clone(), depth+1, max_depth))
			.collect();
			

	//println!("depth: {}:\n {:?}\n", depth, results);


	let (mut ret_moves, mut ret_score) = (Vec::new(), 0);
	match game.board.to_move {
		0 => {
			ret_score = -1000;
			for (mvs, score) in results {
				if score >= ret_score {
					ret_score = score;
					ret_moves = mvs;
				}
			}
		},
		1 => {
			ret_score = 1000;
			for (mvs, score) in results {
				if score <= ret_score {
					ret_score = score;
					ret_moves = mvs;
				}
			}
		},
		_ => () //cant happen
	}

	ret_moves.push(game.mv_from_parent.clone().unwrap());
	(ret_moves, ret_score)
}

impl GameState {
	fn score(&self) -> isize {
		match self.board.to_move {
			0 => if self.board.current_player_wins() { return 1000 },
			1 => if self.board.current_player_wins() { return -1000 },
			_ => (), //cannot happen
		}
		let mut sum: isize = 0;
		sum += self.board.walls_left[0] as isize - self.board.walls_left[1] as isize;
		sum += 2*(self.board.dist_to_goal(1).unwrap() as isize - self.board.dist_to_goal(0).unwrap() as isize);
		if self.board.to_move == 0 { sum += 1 } else { sum -= 1 };
		sum
	}

	fn score_5x5(&self) -> isize {
		match self.board.to_move {
			0 => if self.board.current_player_wins_5x5() { return 1000 },
			1 => if self.board.current_player_wins_5x5() { return -1000 },
			_ => (), //cannot happen
		}
		let mut sum: isize = 0;
		sum += self.board.walls_left[0] as isize - self.board.walls_left[1] as isize;
		sum += 2*(self.board.dist_to_goal(1).unwrap() as isize - self.board.dist_to_goal(0).unwrap() as isize);
		if self.board.to_move == 0 { sum += 1 } else { sum -= 1 };
		sum
	}

}

fn filter_5x5(mv: &Move) -> bool {
	match mv {
		Step(_) | Wall([_, 2..=5, 2..=5]) => true,
		_ => false,
	}
}

pub fn brute_force_5x5(mut game: GameState, max_depth: usize, notation: Notation) -> isize {
	let begin_time = SystemTime::now();

	let results: Vec<_> = 
		game.board
			.all_legal_moves()
			.iter()
			.filter(|mv| filter_5x5(mv))
			.map(|mv| 
				GameState {
					board: game.board.mv_new_no_memory(mv),
					mv_from_parent: Some(mv.clone()),
				})
			.collect::<Vec<_>>()
			.par_iter()
			.map(move |game| _brute_force_recursive_dfs_5x5(game.clone(), 1, max_depth))
			.collect();

	let mut score = 0;
	match game.board.to_move {
		0 => score = *results.iter().map(|(_seq, score)| score).max().unwrap(),
		1 => score = *results.iter().map(|(_seq, score)| score).min().unwrap(),
		_ => (), //cant happen
	};

	let total_time = SystemTime::now().duration_since(begin_time).unwrap().as_secs();
	println!("    That took about {} seconds.\n", total_time);

	print!("    Best sequence found:    ");
	for (seq, sc) in results.iter() {
		if *sc == score {
			for m in seq.iter().rev() {
				print!("{} ", string_from_move(&m, game.board.players[game.board.to_move], notation));
			}
			break;
		}
	}
	println!("");

	score
}

fn _brute_force_recursive_dfs_5x5(
mut game: GameState,
depth: usize,
max_depth: usize) -> (Vec<Move>, isize) {

	if depth == max_depth || game.board.current_player_wins_5x5() {
		return (vec![game.mv_from_parent.clone().unwrap()], game.score_5x5());
	}

	let results: Vec<_> =
		game.board
			.all_legal_moves()
			.iter()
			.filter(|mv| filter_5x5(mv))
			.map(|mv| 
				GameState {
					board: game.board.mv_new_no_memory(mv),
					mv_from_parent: Some(mv.clone()),
				})
			.collect::<Vec<_>>()
			.par_iter()
			.map(move |game| _brute_force_recursive_dfs_5x5(game.clone(), depth+1, max_depth))
			.collect();
			

	//println!("depth: {}:\n {:?}\n", depth, results);


	let (mut ret_moves, mut ret_score) = (Vec::new(), 0);
	match game.board.to_move {
		0 => {
			ret_score = -1000;
			for (mvs, score) in results {
				if score >= ret_score {
					ret_score = score;
					ret_moves = mvs;
				}
			}
		},
		1 => {
			ret_score = 1000;
			for (mvs, score) in results {
				if score <= ret_score {
					ret_score = score;
					ret_moves = mvs;
				}
			}
		},
		_ => () //cant happen
	}

	ret_moves.push(game.mv_from_parent.clone().unwrap());
	(ret_moves, ret_score)
}
