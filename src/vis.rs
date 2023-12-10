pub mod printing {
	use crate::Color;
	use crate::Board;
	use colored::ColoredString;
	use colored::Colorize;

	fn _c (x: usize, y: usize, players: &Vec<[usize; 2]>, walls: &[[[Color; 8]; 8]; 2], squares: &[[Color; 9]; 9]) -> ColoredString {
		match (x%8 == 7, y%4 == 3) {
			(true, true) => {
				match &walls[0][y/4][x/8] {
					Color::Empty => 
						match &walls[1][y/4][x/8] {
							Color::Empty => return "+".into(),
							Color::White => return "|".black().on_white(),
							Color::Red => return "|".black().on_red(),
							Color::Green => return "|".black().on_green(),
							Color::Blue => return "|".black().on_blue(),
						},
					Color::White => return "-".black().on_white(),
					Color::Red => return "-".black().on_red(),
					Color::Green => return "-".black().on_green(),
					Color::Blue => return "-".black().on_blue(),
				}
			},
			(true, false) =>
				match (y/4 == 0, y/4 == 8) {
					(true, true) => "oop".into(),
					(true, false) => {
						match &walls[1][0][x/8] {
							Color::Empty => return ".".into(),
							Color::White => return "|".black().on_white(),
							Color::Red => return "|".black().on_red(),
							Color::Green => return "|".black().on_green(),
							Color::Blue => return "|".black().on_blue(),
						}
					},
					(false, true) => {
						match &walls[1][7][x/8] {
							Color::Empty => return ".".into(),
							Color::White => return "|".black().on_white(),
							Color::Red => return "|".black().on_red(),
							Color::Green => return "|".black().on_green(),
							Color::Blue => return "|".black().on_blue(),
						}
					},
					(false, false) => {
						match &walls[1][y/4][x/8] {
							Color::Empty => (),
							Color::White => return "|".black().on_white(),
							Color::Red => return "|".black().on_red(),
							Color::Green => return "|".black().on_green(),
							Color::Blue => return "|".black().on_blue(),
						};
						match &walls[1][y/4 - 1][x/8] {
							Color::Empty => return ".".into(),
							Color::White => return "|".black().on_white(),
							Color::Red => return "|".black().on_red(),
							Color::Green => return "|".black().on_green(),
							Color::Blue => return "|".black().on_blue(),
						};
					}
				},
			(false, true) => 
				match (x/8 == 0, x/8 == 8) {
					(true, true) => "oops".into(),
					(true, false) => 
						match &walls[0][y/4][0] {
							Color::Empty => return ".".into(),
							Color::White => return "-".black().on_white(),
							Color::Red => return "-".black().on_red(),
							Color::Green => return "-".black().on_green(),
							Color::Blue => return "-".black().on_blue(),
						},
					(false, true) =>
						match &walls[0][y/4][7] {
							Color::Empty => return ".".into(),
						Color::White => return "-".black().on_white(),
						Color::Red => return "-".black().on_red(),
						Color::Green => return "-".black().on_green(),
						Color::Blue => return "-".black().on_blue(),
					},
					(false, false) => {
						match &walls[0][y/4][x/8] {
							Color::Empty => (),
							Color::White => return "-".black().on_white(),
							Color::Red => return "-".black().on_red(),
							Color::Green => return "-".black().on_green(),
							Color::Blue => return "-".black().on_blue(),
						};
						match &walls[0][y/4][x/8-1] {
							Color::Empty => return ".".into(),
							Color::White => return "-".black().on_white(),
							Color::Red => return "-".black().on_red(),
							Color::Green => return "-".black().on_green(),
							Color::Blue => return "-".black().on_blue(),
						};
					}
				},
			(false, false) => {
				if x%8 == 3 && y%4 == 1 {
					match players.iter().position(|&p| p == [y/4, x/8]) {
						None => match &squares[y/4][x/8] {
							Color::Empty => return " ".into(),
							Color::White => return " ".on_white(),
							Color::Red => return " ".on_red(),
							Color::Green => return " ".on_green(),
							Color::Blue => return " ".on_blue(),
						},
						Some(i) => match &squares[y/4][x/8] {
							Color::Empty => return i.to_string().bold(),
							Color::White => return i.to_string().on_white().bold(),
							Color::Red => return i.to_string().on_red().bold(),
							Color::Green => return i.to_string().on_green().bold(),
							Color::Blue => return i.to_string().on_blue().bold(),
						},
					};
				} else {
					match &squares[y/4][x/8] {
							Color::Empty => return " ".into(),
							Color::White => return " ".on_white(),
							Color::Red => return " ".on_red(),
							Color::Green => return " ".on_green(),
							Color::Blue => return " ".on_blue(),
					}
				}
			}
		}
	}
	pub fn show(players: Vec<[usize; 2]>, walls: [[[Color; 8]; 8]; 2], squares: [[Color; 9]; 9]) {
		print!("       --------a-------b-------c-------d-------e-------f-------g-------h--------\n");
		for y in 0..35 {
			if y%4 == 3 {
				print!("       {}", y/4+1);
			} else {print!("       |");};
			for x in 0..71 {
				print!("{}", _c(x,y,&players, &walls, &squares));
			}
			if y%4 == 3 {
				print!("{}\n", y/4+1);
			} else {print!("|\n");};
		}
		print!("       --------a-------b-------c-------d-------e-------f-------g-------h--------\n");
	}

	fn _default_wall_colors(walls: &[[[bool; 8]; 8]; 2]) -> [[[Color; 8]; 8]; 2] {
		let mut res = [[[Color::Empty; 8]; 8]; 2];
		for i in 0..=1 {
			for y in 0..=7 {
				for x in 0..=7 {
					if walls[i][y][x] {res[i][y][x] = Color::White};
				}
			}
		}
		res
	}

	pub fn print_board(b: &Board) {
		show(b.players.to_vec(), _default_wall_colors(&b.walls), [[Color::Empty; 9]; 9]);
	}
}