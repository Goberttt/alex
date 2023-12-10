pub mod move_conversions {
	use crate::Move;

	pub fn move_from_str(s: &str) -> Result<Move, String> {
		match s {
			"N" => return Ok(Move::Step(1)),
			"E" => return Ok(Move::Step(2)),
			"S" => return Ok(Move::Step(3)),
			"W" => return Ok(Move::Step(4)),
			"NN" => return Ok(Move::Step(11)),
			"NE" => return Ok(Move::Step(12)),
			"NW" => return Ok(Move::Step(14)),
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
				_ => return Err(format!("Error: {} is not a valid move.", s)),
			},
			None => return Err(format!("Error: {} is not a valid move.", s)),
		};
		match s.chars().nth(1) {
			Some(c) => match c {
				'a' => y = 0,
				'b' => y = 1,
				'c' => y = 2,
				'd' => y = 3,
				'e' => y = 4,
				'f' => y = 5,
				'g' => y = 6,
				'h' => y = 7,
				_ => return Err(format!("Error: {} is not a valid move.", s)),
			},
			None => return Err(format!("Error: {} is not a valid move.", s)),
		};
		match s.chars().nth(2) {
			Some(c) => match c {
				'1' => x = 0,
				'2' => x = 1,
				'3' => x = 2,
				'4' => x = 3,
				'5' => x = 4,
				'6' => x = 5,
				'7' => x = 6,
				'8' => x = 7,
				_ => return Err(format!("Error: {} is not a valid move.", s)),
			},
			None => return Err(format!("Error: {} is not a valid move.", s)),
		};
		Ok(Move::Wall([i,x,y]))
	}
}