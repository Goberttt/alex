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
	let mut s = 1;
	if game.board.to_move == 1 { s = -1 };

	let res: Result<Vec<(Vec<Move>, isize)>, (Vec<Move>, isize)> =
		game.board
			.all_legal_moves()
			.par_iter()
			.map(|mv|
				_brute_force_recursive_dfs(
				GameState {
					board: game.board.mv_new_no_memory(mv),
					mv_from_parent: Some(mv.clone()),
				},
				1,
				max_depth))
			.collect();

	let (seq, score);
	match res {
		Ok(v) => (seq, score) = v.into_iter().max_by_key(|(_seq, score)| s*score).unwrap(),
		Err((se, sc)) => (seq, score) = (se, sc),
	}

	let total_time = SystemTime::now().duration_since(begin_time).unwrap().as_secs();

	println!("    That took about {} seconds.\n", total_time);

	print!("    Best sequence found:    ");
	for m in seq.iter().rev() {
		print!("{} ", string_from_move(&m, game.board.players[game.board.to_move], notation));
	}
	println!("");

	score
}

fn _brute_force_recursive_dfs(
mut game: GameState,
depth: usize,
max_depth: usize) -> Result<(Vec<Move>, isize), (Vec<Move>, isize)> {

	if depth == max_depth || game.board.current_player_wins() {
		return Ok((vec![game.mv_from_parent.clone().unwrap()], game.score()));
	}

	let mut s = 1;
	if game.board.to_move == 1 { s = -1 };

	let res: Result<Vec<(Vec<Move>, isize)>, (Vec<Move>, isize)> =
		game.board
			.all_legal_moves()
			.par_iter()
			.map(|mv|
				_brute_force_recursive_dfs(
				GameState {
					board: game.board.mv_new_no_memory(mv),
					mv_from_parent: Some(mv.clone()),
				},
				depth+1,
				max_depth))
			.collect();
	let (mut seq, score);
	match res {
		Ok(v) => (seq, score) = v.into_iter().max_by_key(|(_seq, score)| s*score).unwrap(),
		Err((se, sc)) => (seq, score) = (se, sc),
	}

	seq.push(game.mv_from_parent.clone().unwrap());

	match (game.board.to_move == 0 && score == -1000) || (game.board.to_move == 1 && score == 1000) {
		true => Err((seq, score)),
		false => Ok((seq, score)),
	}
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
	let mut s = 1;
	if game.board.to_move == 1 { s = -1 };

	let res: Result<Vec<(Vec<Move>, isize)>, (Vec<Move>, isize)> = 
		game.board
			.all_legal_moves()
			.par_iter()
			.filter(|mv| filter_5x5(mv))
			.map(|mv|
				_brute_force_recursive_dfs_5x5(
				GameState {
					board: game.board.mv_new_no_memory(mv),
					mv_from_parent: Some(mv.clone()),
				},
				1,
				max_depth))
			.collect();

	let (seq, score);
	match res {
		Ok(v) => (seq, score) = v.into_iter().max_by_key(|(_seq, score)| s*score).unwrap(),
		Err((se, sc)) => (seq, score) = (se, sc),
	}

	let total_time = SystemTime::now().duration_since(begin_time).unwrap().as_secs();

	println!("    That took about {} seconds.\n", total_time);

	print!("    Best sequence found:    ");
	for m in seq.iter().rev() {
		print!("{} ", string_from_move(&m, game.board.players[game.board.to_move], notation));
	}		
	println!("");

	score
}

fn _brute_force_recursive_dfs_5x5(
mut game: GameState,
depth: usize,
max_depth: usize) -> Result<(Vec<Move>, isize), (Vec<Move>, isize)> {

	if depth == max_depth || game.board.current_player_wins_5x5() {
		return Ok((vec![game.mv_from_parent.clone().unwrap()], game.score_5x5()));
	}

	let mut s = 1;
	if game.board.to_move == 1 { s = -1 };

	let res: Result<Vec<(Vec<Move>, isize)>, (Vec<Move>, isize)> = 
		game.board
			.all_legal_moves()
			.par_iter()
			.filter(|mv| filter_5x5(mv))
			.map(|mv|
				_brute_force_recursive_dfs_5x5(
				GameState {
					board: game.board.mv_new_no_memory(mv),
					mv_from_parent: Some(mv.clone()),
				},
				depth+1,
				max_depth))
			.collect();
	let (mut seq, score);
	match res {
		Ok(v) => (seq, score) = v.into_iter().max_by_key(|(_seq, score)| s*score).unwrap(),
		Err((se, sc)) => (seq, score) = (se, sc),
	}

	seq.push(game.mv_from_parent.clone().unwrap());

	match (game.board.to_move == 0 && score == -1000) || (game.board.to_move == 1 && score == 1000) {
		true => Err((seq, score)),
		false => Ok((seq, score)),
	}
}
