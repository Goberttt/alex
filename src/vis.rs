pub mod printing {
	use crate::enums::Color;
	use crate::enums::Color::*;
	use crate::InteractiveInstance;
	use crate::enums::Flag::*;
	use crate::enums::Notation;
	use crate::enums::Notation::*;

	use colored::ColoredString;
	use colored::Colorize;

	fn _c (x: usize, y: usize, players: &Vec<[usize; 2]>, walls: &[[[Color; 8]; 8]; 2], squares: &[[Color; 9]; 9]) -> ColoredString {
		match (x%8 == 7, y%4 == 3) {
			(true, true) => {
				match &walls[0][x/8][y/4] {
					Empty => 
						match &walls[1][x/8][y/4] {
							Empty => return "+".into(),
							White => return "|".black().on_white(),
							Red => return "|".black().on_red(),
							Green => return "|".black().on_green(),
							Blue => return "|".black().on_blue(),
						},
					White => return "-".black().on_white(),
					Red => return "-".black().on_red(),
					Green => return "-".black().on_green(),
					Blue => return "-".black().on_blue(),
				}
			},
			(true, false) =>
				match (y/4 == 0, y/4 == 8) {
					(true, true) => "oop".into(),
					(true, false) => {
						match &walls[1][x/8][0] {
							Empty => return ".".into(),
							White => return "|".black().on_white(),
							Red => return "|".black().on_red(),
							Green => return "|".black().on_green(),
							Blue => return "|".black().on_blue(),
						}
					},
					(false, true) => {
						match &walls[1][x/8][7] {
							Empty => return ".".into(),
							White => return "|".black().on_white(),
							Red => return "|".black().on_red(),
							Green => return "|".black().on_green(),
							Blue => return "|".black().on_blue(),
						}
					},
					(false, false) => {
						match &walls[1][x/8][y/4] {
							Empty => (),
							White => return "|".black().on_white(),
							Red => return "|".black().on_red(),
							Green => return "|".black().on_green(),
							Blue => return "|".black().on_blue(),
						};
						match &walls[1][x/8][y/4 - 1] {
							Empty => return ".".into(),
							White => return "|".black().on_white(),
							Red => return "|".black().on_red(),
							Green => return "|".black().on_green(),
							Blue => return "|".black().on_blue(),
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
							Red => return "-".black().on_red(),
							Green => return "-".black().on_green(),
							Blue => return "-".black().on_blue(),
						},
					(false, true) =>
						match &walls[0][7][y/4] {
							Empty => return ".".into(),
						White => return "-".black().on_white(),
						Red => return "-".black().on_red(),
						Green => return "-".black().on_green(),
						Blue => return "-".black().on_blue(),
					},
					(false, false) => {
						match &walls[0][x/8][y/4] {
							Empty => (),
							White => return "-".black().on_white(),
							Red => return "-".black().on_red(),
							Green => return "-".black().on_green(),
							Blue => return "-".black().on_blue(),
						};
						match &walls[0][x/8-1][y/4] {
							Empty => return ".".into(),
							White => return "-".black().on_white(),
							Red => return "-".black().on_red(),
							Green => return "-".black().on_green(),
							Blue => return "-".black().on_blue(),
						};
					}
				},
			(false, false) => {
				if x%8 == 3 && y%4 == 1 {
					match players.iter().position(|&p| p == [x/8, y/4]) {
						None => match &squares[x/8][y/4] {
							Empty => return " ".into(),
							White => return " ".on_white(),
							Red => return " ".on_red(),
							Green => return " ".on_green(),
							Blue => return " ".on_blue(),
						},
						Some(i) => match &squares[x/8][y/4] {
							Empty => return (i+1).to_string().bold(),
							White => return (i+1).to_string().on_white().bold(),
							Red => return (i+1).to_string().on_red().bold(),
							Green => return (i+1).to_string().on_green().bold(),
							Blue => return (i+1).to_string().on_blue().bold(),
						},
					};
				} else {
					match &squares[x/8][y/4] {
							Empty => return " ".into(),
							White => return " ".on_white(),
							Red => return " ".on_red(),
							Green => return " ".on_green(),
							Blue => return " ".on_blue(),
					}
				}
			}
		}
	}
	fn show(players: Vec<[usize; 2]>,
			walls: [[[Color; 8]; 8]; 2],
			squares: [[Color; 9]; 9],
			invert: bool,
			walls_left: [usize; 2],
			notation: Notation) {
		match notation {
			Relative => {
				print!("       --------a-------b-------c-------d-------e-------f-------g-------h--------\n");
				for yy in (0..35).rev() {
					let y = if invert { 34-yy } else { yy };
					if y%4 == 3 {
						print!("       {}", y/4+1);
					} else {print!("       |");};
					for x in 0..71 {
						print!("{}", _c(x, y, &players, &walls, &squares));
					}
					if y%4 == 3 {
						print!("{}\n", y/4+1);
					} else { print!("|\n"); };
				}
				print!("       --------a-------b-------c-------d-------e-------f-------g-------h--------\n\n");
				println!("         Walls:      Player 1: {}               Player 2: {}", walls_left[0], walls_left[1]);
			},
			Absolute => {
				print!("       ----a-------b-------c-------d-------e-------f-------g-------h-------i----\n");
				for yy in (0..35).rev() {
					let y = if invert { 34-yy } else { yy };
					if y%4 == 1 {
						print!("       {}", y/4+1);
					} else {print!("       |");};
					for x in 0..71 {
						print!("{}", _c(x, y, &players, &walls, &squares));
					}
					if y%4 == 1 {
						print!("{}\n", y/4+1);
					} else { print!("|\n"); };
				}
				print!("       ----a-------b-------c-------d-------e-------f-------g-------h-------i----\n\n");
				println!("         Walls:      Player 1: {}               Player 2: {}", walls_left[0], walls_left[1]);
			}
		}
		
	}

	fn _default_wall_colors(walls: &[[[bool; 8]; 8]; 2]) -> [[[Color; 8]; 8]; 2] {
		let mut res = [[[Empty; 8]; 8]; 2];
		for i in 0..=1 {
			for y in 0..=7 {
				for x in 0..=7 {
					if walls[i][y][x] {res[i][y][x] = White};
				}
			}
		}
		res
	}

	pub fn print_board(ii: &InteractiveInstance) {
		let b = &ii.board;
		let invert = ii.flags.get(&Invert).unwrap();
		show(b.players.to_vec(), _default_wall_colors(&b.walls), [[Empty; 9]; 9], *invert, [b.walls_left[0], b.walls_left[1]], ii.notation.clone());
	}
}