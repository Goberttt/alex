use crate::enums::{ Notation, IoState, Flag, ParseError, HelpMessage };
use crate::enums::Notation::*;
use crate::enums::IoState::*;
use crate::enums::Flag::*;
use crate::enums::ParseError::*;

use std::io;
use std::collections::HashMap;

pub mod move_conversions {
	use crate::enums::ParseError;
	use crate::enums::ParseError::*;
	use crate::enums::Move;
	use crate::enums::Move::*;
	use crate::enums::Notation;
	use crate::enums::Notation::*;

	pub fn move_from_str(s: &str, [a, b]: [usize; 2], [m, n]: [usize; 2], notation: Notation) -> Result<Move, ParseError> {
	//a, b is current players position
	//m, n is other players position
		match notation {
			Relative => {
				match s {
					"N"  => return Ok(Step(1) ),
					"E"  => return Ok(Step(2) ),
					"S"  => return Ok(Step(3) ),
					"W"  => return Ok(Step(4) ),
					"NN" => return Ok(Step(11)),
					"NE" => return Ok(Step(12)),
					"NW" => return Ok(Step(14)),
					"EN" => return Ok(Step(21)),
					"EE" => return Ok(Step(22)),
					"ES" => return Ok(Step(23)),
					"WN" => return Ok(Step(41)),
					"WS" => return Ok(Step(43)),
					"WW" => return Ok(Step(44)),
					"SS" => return Ok(Step(33)),
					"SE" => return Ok(Step(32)),
					"SW" => return Ok(Step(34)),
					_ => (),
				}

				let (i, x, y);
				match s.chars().nth(0) {
					Some(c) => match c {
						'H' => i = 0,
						'V' => i = 1,
						_ => return Err(InvalidMove),
					},
					None => return Err(InvalidMove),
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
						_ => return Err(InvalidMove),
					},
					None => return Err(InvalidMove),
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
						_ => return Err(InvalidMove),
					},
					None => return Err(InvalidMove),
				};
				Ok(Move::Wall([i, x, y]))
			},
			Absolute => {
				let (x, y);
				match s.chars().nth(0) {
					Some(c) => match c {
						'a' => x = 0,
						'b' => x = 1,
						'c' => x = 2,
						'd' => x = 3,
						'e' => x = 4,
						'f' => x = 5,
						'g' => x = 6,
						'h' => x = 7,
						'i' => x = 8,
						_ => return Err(InvalidMove),
					},
					None => return Err(InvalidMove),
				};
				match s.chars().nth(1) {
					Some(c) => match c {
						'1' => y = 0,
						'2' => y = 1,
						'3' => y = 2,
						'4' => y = 3,
						'5' => y = 4,
						'6' => y = 5,
						'7' => y = 6,
						'8' => y = 7,
						'9' => y = 8,
						_ => return Err(InvalidMove),
					},
					None => return Err(InvalidMove),
				};
				match (x-(a as isize), y-(b as isize)) {
					(0,1)  => return Ok(Step(1) ),
					(1,0)  => return Ok(Step(2) ),
					(0,-1) => return Ok(Step(3) ),
					(-1,0) => return Ok(Step(4) ),
					(0,2)  => return Ok(Step(11)),
					(2,0)  => return Ok(Step(22)),
					(0,-2) => return Ok(Step(33)),
					(-2,0) => return Ok(Step(44)),
					(1,1) => {
						match ((m as isize)-(a as isize), (n as isize)-(b as isize)) {
							(1,0) => return Ok(Step(12)),
							_ => return Ok(Step(21)),
						}
					},
					(1,-1) => {
						match ((m as isize)-(a as isize), (n as isize)-(b as isize)) {
							(1,0) => return Ok(Step(14)),
							_ => return Ok(Step(41)),
						}
					},
					(-1,1) => {
						match ((m as isize)-(a as isize), (n as isize)-(b as isize)) {
							(-1,0) => return Ok(Step(41)),
							_ => return Ok(Step(14)),
						}
					},
					(-1,-1) => {
						match ((m as isize)-(a as isize), (n as isize)-(b as isize)) {
							(-1,0) => return Ok(Step(42)),
							_ => return Ok(Step(24)),
						}
					},
					_ => (),
				};
				let (i, x, y);
				match s.chars().nth(3) {
					Some(c) => match c {
						'h' => i = 0,
						'v' => i = 1,
						_ => return Err(InvalidMove),
					},
					None => return Err(InvalidMove),
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
						_ => return Err(InvalidMove),
					},
					None => return Err(InvalidMove),
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
						_ => return Err(InvalidMove),
					},
					None => return Err(InvalidMove),
				};
				Ok(Move::Wall([i, x, y]))
			}
		}
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


pub fn help(s: &Option<String>, help_texts: &HashMap<HelpMessage, String>) {
    match s {
        None => println!{"    {}", help_texts.get(&HelpMessage::General).unwrap()},
        Some(c) => match c.as_str() {
            "help" => println!("    {}", help_texts.get(&HelpMessage::General).unwrap()),
            "new" => println!("    {}", help_texts.get(&HelpMessage::New).unwrap()),
            "move" => println!("    {}", help_texts.get(&HelpMessage::Move).unwrap()),
            "show" => println!("    {}", help_texts.get(&HelpMessage::Show).unwrap()),
            "encoding" => println!("    {}", help_texts.get(&HelpMessage::Encoding).unwrap()),
            "brute" => println!("    {}", help_texts.get(&HelpMessage::Brute).unwrap()),
            "set" => println!("    {}", help_texts.get(&HelpMessage::Set).unwrap()),
            "unset" => println!("    {}", help_texts.get(&HelpMessage::Unset).unwrap()),
            _ => println!("    {}", help_texts.get(&HelpMessage::WrongInput).unwrap()),
        }
    }
}

fn _get_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input. Please only enter valid UTF-8.");
    input.trim().splitn(2, " ").map(|x| x.to_string()).collect()
}

pub fn get_and_parse_input() -> Result<IoState, ParseError> {
    let new = _get_input();

    //parse the input and set the variables

    match (new.get(0), new.get(1)) {
        (Some(s), None) => match s.as_str() {
            "help" | "h" => return Ok(Help(None)),
            "new" | "n" => return Ok(NewBoard(None)),
            "move" | "m" => return Err(NoMovesGiven),
            "_move_no_check" | "_mnc" => return Err(NoMovesGiven),
            "show" | "s" => return Ok(ShowBoard),
            "exit" | "end" | "quit" => return Ok(Quit),
            "set" => return Err(NoFlagGiven),
            "unset" => return Err(NoFlagGiven),
            "notation" => return Err(NoNotationGiven),
            "brute" | "b" => return Ok(Brute),
            "_fill" => return Err(NoFillChordsGiven),
            _ => return Err(UnknownCommand),
            },
        (Some(s), Some(i)) => match s.as_str() {
                "help" | "h" => return Ok(Help(Some((*i.clone()).to_string()))),
                "new" | "n" => return Ok(NewBoard(Some(i.clone()))),
                "move" | "m" => return Ok(PlayMoves(i.clone())),
                "_move_no_check" | "_mnc" => return Ok(PlayMovesNoCheck(i.clone())),
                "show" | "s" => return Err(InputAfterShow),
                "exit" | "end" | "quit" => return Ok(Quit),
                "set" => return Ok(Set(_flag_from_str(i.clone())?)),
                "unset" => return Ok(Unset(_flag_from_str(i.clone())?)),
                "notation" => return Ok(SetNotation(_notation_from_str(i.clone())?)),
                "brute" | "b" => return Err(InputAfterBrute),
                "_fill" => return Ok(Fill(i.clone())),
                _ => return Err(UnknownCommand),
            },
        _ => return Err(UnknownCommand),
    };
}

fn _flag_from_str(s: String) -> Result<Flag, ParseError> {
	match s.as_str() {
		"invert" => Ok(Invert),
		_ => Err(UnknownFlag),
	}
}

fn _notation_from_str(s: String) -> Result<Notation, ParseError> {
	match s.as_str() {
		"absolute" => Ok(Absolute),
		"relative" => Ok(Relative),
		_ => Err(UnknownNotation),
	}
}