use std::io;

pub mod move_conversions {

	use crate::enums::Move;

	pub fn move_from_str(s: &str) -> Result<Move, String> {
		match s {
			"N"  => return Ok(Move::Step(1) ),
			"E"  => return Ok(Move::Step(2) ),
			"S"  => return Ok(Move::Step(3) ),
			"W"  => return Ok(Move::Step(4) ),
			"NN" => return Ok(Move::Step(11)),
			"NE" => return Ok(Move::Step(12)),
			"NW" => return Ok(Move::Step(14)),
			"EN" => return Ok(Move::Step(21)),
			"EE" => return Ok(Move::Step(22)),
			"ES" => return Ok(Move::Step(23)),
			"WN" => return Ok(Move::Step(41)),
			"WS" => return Ok(Move::Step(43)),
			"WW" => return Ok(Move::Step(44)),
			"SS" => return Ok(Move::Step(33)),
			"SE" => return Ok(Move::Step(32)),
			"SW" => return Ok(Move::Step(34)),
			_ => (),
		}
		let i;
		let x;
		let y;
		match s.chars().nth(0) {
			Some(c) => match c {
				'H' => i = 0,
				'V' => i = 1,
				_ => return Err(format!("{} is not a valid move. Type help encoding for help.", s)),
			},
			None => return Err(format!("{} is not a valid move. Type help encoding for help.", s)),
		};
		match s.chars().nth(1) {
			Some(c) => match c {
				'a' => x = 0,
				'b' => x = 1,
				'c' => x = 2,
				'd' => x = 3,
				'e' => x = 4,
				'f' => x = 5,
				'g' => x = 6,
				'h' => x = 7,
				_ => return Err(format!("{} is not a valid move. Type help encoding for help.", s)),
			},
			None => return Err(format!("{} is not a valid move. Type help encoding for help.", s)),
		};
		match s.chars().nth(2) {
			Some(c) => match c {
				'1' => y = 0,
				'2' => y = 1,
				'3' => y = 2,
				'4' => y = 3,
				'5' => y = 4,
				'6' => y = 5,
				'7' => y = 6,
				'8' => y = 7,
				_ => return Err(format!("{} is not a valid move. Type help encoding for help.", s)),
			},
			None => return Err(format!("{} is not a valid move. Type help encoding for help.", s)),
		};
		Ok(Move::Wall([i, x, y]))
	}

	pub fn string_from_move(mv: &Move) -> String {
		match mv {
			Move::Step(1)   => return format!("N") ,
			Move::Step(2) 	=> return format!("E") ,
			Move::Step(3) 	=> return format!("S") ,
			Move::Step(4) 	=> return format!("W") ,
			Move::Step(11)	=> return format!("NN"), 
			Move::Step(12)	=> return format!("NE"),
			Move::Step(14)	=> return format!("NW"),
			Move::Step(21)	=> return format!("EN"),
			Move::Step(22)	=> return format!("EE"),
			Move::Step(23)	=> return format!("ES"),
			Move::Step(41)	=> return format!("WN"),
			Move::Step(43)	=> return format!("WS"),
			Move::Step(44)	=> return format!("WW"),
			Move::Step(33)	=> return format!("SS"),
			Move::Step(32)	=> return format!("SE"),
			Move::Step(34)	=> return format!("SW"),
			Move::Step(_)   => return format!(" "), //cant happen
			Move::Wall([i, x, y]) => return format!("{}{}{}", _char_from_i(i), _char_from_x(x), y+1),
		}
	}

	fn _char_from_i(i: &usize) -> char {
		match i {
			0 => return 'H',
			1 => return 'V',
			_ => return ' ', //cant happen
		}
	}

	fn _char_from_x(x: &usize) -> char {
		match x {
			0 => return 'a',
			1 => return 'b',
			2 => return 'c',
			3 => return 'd',
			4 => return 'e',
			5 => return 'f',
			6 => return 'g',
			7 => return 'h',
			_ => return ' ', //cant happen
		}
	}
}


pub fn help(s: &Option<String>) {
    match s {
        None => println!{"The following commands are supported:
help, new, move, show and exit. Type help <command> to receive help about a command.
Type help encoding to see help about the move encoding."},
        Some(c) => match c.as_str() {
            "help" => println!("Type help <command> to receive help about a command.
Example: >> help new.
Type help encoding to see help about the move encoding."),
            "new" => println!("Resets the board.
This command can be followed by a valid move sequence like this: >> new S N S N."),
            "move" => println!("Must be followed by a valid move sequence.
Applies this move sequence to the current board.
Example: >> move S N Hd3."),
            "show" => println!("Prints the current board."),
            "exit" => println!("Exits the application."),
            "encoding" => println!("The moves are encoded as follows:
Moving the pawn is encoded by cardinal directions, i.e. N, E, S, W.
A jump is encoded by two consequtive such directions, like NN or NW for a sideways jump.
Walls are encoded by H and V standing for horizontal and vertical.
This is followed by the coordinates of the center of the wall, like this: Hd3 He7 Va1 Vb5."),
            _ => {println!("I'm sorry, I don't understand the input."); help(&None);},
        }
    }
}

pub fn get_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input. Please only enter valid UTF-8.");
    input.trim().splitn(2, " ").map(|x| x.to_string()).collect()
}

