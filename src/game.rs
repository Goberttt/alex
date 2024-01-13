use crate::Board;
use std::collections::HashSet;

pub struct GameState {
	pub board: Board,
	pub children: Vec<usize>,
	pub parent: Option<usize>,
	pub is_winning: bool,
	pub is_loosing: bool,
}

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
		if arena.len()%100000 == 0 { println!("{}", arena.len()) };
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