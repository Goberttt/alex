//contains functionality for printing the board. Will be expanded to make coloring possible

use crate::InteractiveInstance;
use crate::enums::{ Color, Color::*, Notation, Notation::*, Move, Move::*, Flag::* };
use crate::helpers::string_from_move;

use colored::ColoredString;
use colored::Colorize;

fn _c (x: usize, y: usize, players: &Vec<[usize; 2]>, walls: &[[[Color; 8]; 8]; 2], squares: &[[Color; 9]; 9], to_move: &usize) -> ColoredString {
	match (x%8 == 7, y%4 == 3) {
		(true, true) => {
			match &walls[0][x/8][y/4] {
				Empty => 
					match &walls[1][x/8][y/4] {
						Empty => return "+".into(),
						White => return "|".black().on_white(),
						//Red => return "|".black().on_red(),
						//Green => return "|".black().on_green(),
						//Blue => return "|".black().on_blue(),
					},
				White => return "-".black().on_white(),
				//Red => return "-".black().on_red(),
				//Green => return "-".black().on_green(),
				//Blue => return "-".black().on_blue(),
			}
		},
		(true, false) =>
			match (y/4 == 0, y/4 == 8) {
				(true, true) => "oop".into(),
				(true, false) => {
					match &walls[1][x/8][0] {
						Empty => return ".".into(),
						White => return "|".black().on_white(),
						//Red => return "|".black().on_red(),
						//Green => return "|".black().on_green(),
						//Blue => return "|".black().on_blue(),
					}
				},
				(false, true) => {
					match &walls[1][x/8][7] {
						Empty => return ".".into(),
						White => return "|".black().on_white(),
						//Red => return "|".black().on_red(),
						//Green => return "|".black().on_green(),
						//Blue => return "|".black().on_blue(),
					}
				},
				(false, false) => {
					match &walls[1][x/8][y/4] {
						Empty => (),
						White => return "|".black().on_white(),
						//Red => return "|".black().on_red(),
						//Green => return "|".black().on_green(),
						//Blue => return "|".black().on_blue(),
					};
					match &walls[1][x/8][y/4 - 1] {
						Empty => return ".".into(),
						White => return "|".black().on_white(),
						//Red => return "|".black().on_red(),
						//Green => return "|".black().on_green(),
						//Blue => return "|".black().on_blue(),
					};
				}
			},
		(false, true) => 
			match (x/8 == 0, x/8 == 8) {
				(true, true) => "oops".into(),
				(true, false) => 
					match &walls[0][0][y/4] {
						Empty => return ".".into(),
						White => return "-".black().on_white(),
						//Red => return "-".black().on_red(),
						//Green => return "-".black().on_green(),
						//Blue => return "-".black().on_blue(),
					},
				(false, true) =>
					match &walls[0][7][y/4] {
						Empty => return ".".into(),
					White => return "-".black().on_white(),
					//Red => return "-".black().on_red(),
					//Green => return "-".black().on_green(),
					//Blue => return "-".black().on_blue(),
				},
				(false, false) => {
					match &walls[0][x/8][y/4] {
						Empty => (),
						White => return "-".black().on_white(),
						//Red => return "-".black().on_red(),
						//Green => return "-".black().on_green(),
						//Blue => return "-".black().on_blue(),
					};
					match &walls[0][x/8-1][y/4] {
						Empty => return ".".into(),
						White => return "-".black().on_white(),
						//Red => return "-".black().on_red(),
						//Green => return "-".black().on_green(),
						//Blue => return "-".black().on_blue(),
					};
				}
			},
		(false, false) => {
			if x%8 == 3 && y%4 == 1 {
				match players.iter().position(|&p| p == [x/8, y/4]) {
					None => match &squares[x/8][y/4] {
						Empty => return " ".into(),
						White => return " ".on_white(),
						//Red => return " ".on_red(),
						//Green => return " ".on_green(),
						//Blue => return " ".on_blue(),
					},
					Some(i) => match &squares[x/8][y/4] {
						Empty => if i == *to_move { return (i+1).to_string().bold().blue() } else { return (i+1).to_string().bold() },
						White => if i == *to_move { return (i+1).to_string().on_white().bold().blue() } else { return (i+1).to_string().bold() },
						//Red => if i == *to_move { return (i+1).to_string().on_red().bold().blue() } else { return (i+1).to_string().bold() },
						//Green => if i == *to_move { return (i+1).to_string().on_green().bold().blue() } else { return (i+1).to_string().bold() },
						//Blue => if i == *to_move { return (i+1).to_string().on_blue().bold().blue() } else { return (i+1).to_string().bold() },
					},
				};
			} else {
				match &squares[x/8][y/4] {
						Empty => return " ".into(),
						White => return " ".on_white(),
						//Red => return " ".on_red(),
						//Green => return " ".on_green(),
						//Blue => return " ".on_blue(),
				}
			}
		}
	}
}
fn show(players: Vec<[usize; 2]>,
		to_move: usize,
		walls: [[[Color; 8]; 8]; 2],
		squares: [[Color; 9]; 9],
		invert: bool,
		walls_left: [usize; 2],
		notation: Notation,
		moves: Vec<Move>) {
	match notation {
		Relative => {
			print!("       --------a-------b-------c-------d-------e-------f-------g-------h--------\n");
			for yy in (0..35).rev() {
				let y = if invert { 34-yy } else { yy };
				if y%4 == 3 {
					print!("       {}", y/4+1);
				} else {print!("       |");};
				for x in 0..71 {
					print!("{}", _c(x, y, &players, &walls, &squares, &to_move));
				}
				if y%4 == 3 {
					print!("{}\n", y/4+1);
				} else { print!("|\n"); };
			}
			print!("       --------a-------b-------c-------d-------e-------f-------g-------h--------\n\n");
		},
		Absolute => {
			print!("       ----a-------b-------c-------d-------e-------f-------g-------h-------i----\n");
			for yy in (0..35).rev() {
				let y = if invert { 34-yy } else { yy };
				if y%4 == 1 {
					print!("       {}", y/4+1);
				} else {print!("       |");};
				for x in 0..71 {
					print!("{}", _c(x, y, &players, &walls, &squares, &to_move));
				}
				if y%4 == 1 {
					print!("{}\n", y/4+1);
				} else { print!("|\n"); };
			}
			print!("       ----a-------b-------c-------d-------e-------f-------g-------h-------i----\n\n");
		}
	}
	println!("         Walls:      Player 1: {}               Player 2: {}", walls_left[0], walls_left[1]);
	print!("         Moves:      ");
	let mut char_printed=0;
	for mv in moves {
		print!("{} ", string_from_move(&mv, players[to_move], notation.clone()));
		match (mv, notation) {
			(Step(_), Absolute) => char_printed += 3,
			(Wall(_), Absolute) => char_printed += 4,
			(Step(_), Relative) => char_printed += 2,
			(Wall(_), Relative) => char_printed += 3,
		}
		if char_printed > 50 {
			println!("");
			print!("                     ");
			char_printed = 0;
		}			
	}
	println!("\n\n");
}

fn _default_wall_colors(walls: &u128) -> [[[Color; 8]; 8]; 2] {
	let mut res = [[[Empty; 8]; 8]; 2];
	for i in 0..=1 {
		for y in 0..=7 {
			for x in 0..=7 {
				if walls & (1 << (64*i + 8*x + y)) != 0 {res[i][y][x] = White};
			}
		}
	}
	res
}

pub fn print_board(ii: &InteractiveInstance) {
	let b = &ii.board;
	let invert = ii.flags.get(&Invert).unwrap();
	show(b.players.to_vec(),
		b.to_move_indices().0,
		_default_wall_colors(&b.walls),
		[[Empty; 9]; 9], *invert, [b.walls_left[0],
		b.walls_left[1]], ii.notation.clone(),
		ii.board.move_sequence.clone());

	//println!("score: {}", 2*(ii.board.dist_to_goal(1).unwrap() as isize - ii.board.dist_to_goal(0).unwrap() as isize));
	//println!("dists: {}, {}", ii.board.dist_to_goal(1).unwrap(), ii.board.dist_to_goal(0).unwrap());
}
