use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Tile {
	pub pos: [usize; 2],
	pub neighboors: [bool; 4], //NESW
}

#[derive(Clone)]
pub struct Graph {
	pub vertices: [[Tile; 9]; 9],
}

impl Tile {
	fn new(x: usize, y: usize) -> Tile {
		let n;
		match (x,y) {
			(0,0) => n = [true, true, false, false],
			(0,8) => n = [false, true, true, false],
			(8,0) => n = [true, false, false, true],
			(8,8) => n = [false, false, true, true],
			(0,_) => n = [true, true, true, false],
			(8,_) => n = [true, false, true, true],
			(_,0) => n = [true, true, false, true],
			(_,8) => n = [false, true, true, true],
			_     => n = [true, true, true, true],
		};
		Tile { pos: [x, y], neighboors: n }
	}

	fn empty() -> Tile {
		Tile {pos: [0,0], neighboors: [true,true,true,true]}
	}
}

impl Default for Graph {
	fn default() -> Graph {
		let mut v = [[Tile::empty(); 9]; 9];
		for x in 0..=8 {
			for y in 0..=8 {
				v[x][y] = Tile::new(x,y);
			}
		};
		
		Graph{ vertices: v }
	}
}

impl Graph {
	pub fn place_wall(&mut self, [i, x, y]: [usize; 3]) {
		match i {
			0 => { //horizontal
				self.vertices[x][y].neighboors[0] = false;
				self.vertices[x+1][y].neighboors[0] = false;
				self.vertices[x][y+1].neighboors[2] = false;
				self.vertices[x+1][y+1].neighboors[2] = false;
			},
			1 => { //vertival
				self.vertices[x][y].neighboors[1] = false;
				self.vertices[x+1][y].neighboors[3] = false;
				self.vertices[x][y+1].neighboors[1] = false;
				self.vertices[x+1][y+1].neighboors[3] = false;
			},
			_ => () //cannot happen
		}
	}
	pub fn remove_wall(&mut self, [i, x, y]: [usize; 3]) {
		match i {
			0 => { //horizontal
				self.vertices[x][y].neighboors[0] = true;
				self.vertices[x+1][y].neighboors[0] = true;
				self.vertices[x][y+1].neighboors[2] = true;
				self.vertices[x+1][y+1].neighboors[2] = true;
			},
			1 => { //vertival
				self.vertices[x][y].neighboors[1] = true;
				self.vertices[x+1][y].neighboors[3] = true;
				self.vertices[x][y+1].neighboors[1] = true;
				self.vertices[x+1][y+1].neighboors[3] = true;
			},
			_ => () //cannot happen
		}
	}
	pub fn dist_to_goal(&self, [a, b]: [usize; 2], g: usize) -> Option<usize> {
		//returns the distance to goal g from position [b, a] with extra wall [i, x, y]
		//simple bfs
		let mut front = vec![self.vertices[a][b]];
		let mut found = HashSet::new();
		let mut new_front = Vec::new();
		let mut steps = 0;

		loop {
			steps += 1;
			for tile in front {
				if tile.neighboors[0] && !found.contains(&self.vertices[tile.pos[0]][tile.pos[1]+1]) {
					if tile.pos[1]+1 == g { return Some(steps) };
					found.insert(self.vertices[tile.pos[0]][tile.pos[1]+1]);
					new_front.push(self.vertices[tile.pos[0]][tile.pos[1]+1]);
				};
				if tile.neighboors[1] && !found.contains(&self.vertices[tile.pos[0]+1][tile.pos[1]]) {
					found.insert(self.vertices[tile.pos[0]+1][tile.pos[1]]);
					new_front.push(self.vertices[tile.pos[0]+1][tile.pos[1]]);
				};
				if tile.neighboors[2] && !found.contains(&self.vertices[tile.pos[0]][tile.pos[1]-1]) {
					if tile.pos[1]-1 == g { return Some(steps) };
					found.insert(self.vertices[tile.pos[0]][tile.pos[1]-1]);
					new_front.push(self.vertices[tile.pos[0]][tile.pos[1]-1]);
				};
				if tile.neighboors[3] && !found.contains(&self.vertices[tile.pos[0]-1][tile.pos[1]]) {
					found.insert(self.vertices[tile.pos[0]-1][tile.pos[1]]);
					new_front.push(self.vertices[tile.pos[0]-1][tile.pos[1]]);
				};
			}
			front = new_front.clone();
			new_front.clear();
			if front.len() == 0 { return None };
		}
	}
}
/*
	fn show_graph(g: &Graph) {
		print!("       --------a-------b-------c-------d-------e-------f-------g-------h--------\n");
		for y in (0..35).rev() {
			if y%4 == 3 {
				print!("       {}", y/4+1);
			} else {print!("       |");};
			for x in 0..71 {
				_g(x, y, g);
			}
			if y%4 == 3 {
				print!("{}\n", y/4+1);
			} else { print!("|\n"); };
		}
		print!("       --------a-------b-------c-------d-------e-------f-------g-------h--------\n");
	}

	fn _g(x: usize, y: usize, g: &Graph) {
		match (x%8 == 7, y%4 == 3) {
			(true, true) => print!("+"),
			(true, false) => match y%4 {
				0 => if g.vertices[x/8][y/4].neighboors[1] { print!(">") } else { print!(".") },
				2 => if g.vertices[x/8+1][y/4].neighboors[3] { print!("<") } else { print!(".") },
				_ => print!("."),
			},
			(false, true) => match x%8 {
				2 => if g.vertices[x/8][y/4].neighboors[0] { print!("A") } else { print!(".") },
				4 => if g.vertices[x/8][y/4+1].neighboors[2] {print!("V") } else { print!(".") },
				_ => print!("."),
			}
			_ => print!(" "),
		}
	}
*/