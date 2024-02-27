use crate::Board;
use crate::enums::{ Notation, Move, Move::*, Player::* };
use crate::helpers::string_from_move;

use std::time::SystemTime;

use rayon::prelude::*;

#[derive(Clone)]
pub struct GameState {
	pub board: Board,
	pub mv_from_parent: Option<Move>,
}

pub fn brute_force(mut board: Board, max_depth: usize, notation: Notation) {
	let begin_time = SystemTime::now();

	let s;
	match board.to_move {
		Player1 => s = 1,
		Player2 => s = -1,
	}

	let res: Result<Vec<(Vec<Move>, isize)>, (Vec<Move>, isize)> =
		board
			.all_legal_moves()
			.par_iter()
			.map(|mv|
				_brute_force_recursive_dfs(
				GameState {
					board: board.mv_new_no_memory(mv),
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

	let total_time = SystemTime::now().duration_since(begin_time).unwrap();
	let millis = total_time.as_millis();
	let secs = millis / 1000;
	let mins = secs / 60;
	let hours = mins / 60;

	print!("    That took about ");

	if hours != 0 {
		print!("{} Hours ", hours);
	}
	if mins != 0 || hours != 0 {
		print!("{} Minutes ", mins % 60);
	}
	if (secs != 0 || mins!= 0) && hours == 0 {
		print!("{} Seconds ", secs%60);
	}
	if secs < 15 {
		print!("{} Milliseconds", millis%1000);
	}


	print!("\n    Best sequence found:    ");
	for m in seq.iter().rev() {
		print!("{} ", string_from_move(&m, board.players[board.to_move_indices().0], notation));
	}
	println!("");

	println!("    Score is {}", score);
}

fn _brute_force_recursive_dfs(
mut game: GameState,
depth: usize,
max_depth: usize) -> Result<(Vec<Move>, isize), (Vec<Move>, isize)> {

	if depth == max_depth || game.board.current_player_wins() {
		return Ok((vec![game.mv_from_parent.clone().unwrap()], game.score()));
	}

	let s;
	match game.board.to_move {
		Player1 => s = 1,
		Player2 => s = -1,
	}

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

	match (game.board.to_move == Player1 && score == -1000) || (game.board.to_move == Player2 && score == 1000) {
		true => Err((seq, score)),
		false => Ok((seq, score)),
	}
}

impl GameState {
	fn score(&self) -> isize {
		match self.board.to_move {
			Player1 => if self.board.current_player_wins() { return 1000 },
			Player2 => if self.board.current_player_wins() { return -1000 },
		}
		let mut sum: isize = 0;
		sum += self.board.walls_left[0] as isize - self.board.walls_left[1] as isize;
		sum += 2*(self.board.dist_to_goal(1).unwrap() as isize - self.board.dist_to_goal(0).unwrap() as isize);
		if self.board.to_move == Player1 { sum += 1 } else { sum -= 1 };
		sum
	}

	fn score_5x5(&self) -> isize {
		match self.board.to_move {
			Player1 => if self.board.current_player_wins_5x5() { return 1000 },
			Player2 => if self.board.current_player_wins_5x5() { return -1000 },
		}
		let mut sum: isize = 0;
		sum += self.board.walls_left[0] as isize - self.board.walls_left[1] as isize;
		sum += 2*(self.board.dist_to_goal(1).unwrap() as isize - self.board.dist_to_goal(0).unwrap() as isize);
		if self.board.to_move == Player1 { sum += 1 } else { sum -= 1 };
		sum
	}

}

fn filter_5x5(mv: &Move) -> bool {
	match mv {
		Step(_) | Wall([_, 2..=5, 2..=5]) => true,
		_ => false,
	}
}

pub fn brute_force_5x5(mut board: Board, max_depth: usize, notation: Notation) {
	let begin_time = SystemTime::now();
	
	let s;
	match board.to_move {
		Player1 => s = 1,
		Player2 => s = -1,
	}

	let res: Result<Vec<(Vec<Move>, isize)>, (Vec<Move>, isize)> = 
		board
			.all_legal_moves()
			.par_iter()
			.filter(|mv| filter_5x5(mv))
			.map(|mv|
				_brute_force_recursive_dfs_5x5(
				GameState {
					board: board.mv_new_no_memory(mv),
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

	let total_time = SystemTime::now().duration_since(begin_time).unwrap();
	let millis = total_time.as_millis();
	let secs = millis / 1000;
	let mins = secs / 60;
	let hours = mins / 60;

	print!("    That took about ");

	if hours != 0 {
		print!("{} Hours ", hours);
	}
	if mins != 0 || hours != 0 {
		print!("{} Minutes ", mins%60);
	}
	if (secs != 0 || mins!= 0) && hours == 0 {
		print!("{} Seconds ", secs&60);
	}
	if secs < 15 {
		print!("{} Milliseconds", millis%1000);
	}


	print!("\n    Best sequence found:    ");
	for m in seq.iter().rev() {
		print!("{} ", string_from_move(&m, board.players[board.to_move_indices().0], notation));
	}
	println!("");

	println!("    Score is {}", score);
}

fn _brute_force_recursive_dfs_5x5(
mut game: GameState,
depth: usize,
max_depth: usize) -> Result<(Vec<Move>, isize), (Vec<Move>, isize)> {

	if depth == max_depth || game.board.current_player_wins_5x5() {
		return Ok((vec![game.mv_from_parent.clone().unwrap()], game.score_5x5()));
	}

	let s;
	match game.board.to_move {
		Player1 => s = 1,
		Player2 => s = -1,
	}

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

	match (game.board.to_move == Player1 && score == -1000) || (game.board.to_move == Player2 && score == 1000) {
		true => Err((seq, score)),
		false => Ok((seq, score)),
	}
}
