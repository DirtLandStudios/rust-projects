#![allow(dead_code)]

fn main() {

}

type Position = (u8, u8);

const WIDTH: usize = 9;
const AREA: usize = (WIDTH * WIDTH) as usize;

struct Game {
	board : [u8; AREA],
	fixed: [bool; AREA] //mark numbers that cannot change
}

impl Game {
	pub fn new(import_board: [u8; AREA]) -> Game {
		//check that board is good
		let mut iter_board = import_board.iter();
		match iter_board.find(|&x| x >= &10) {
			Some(_n) => {
				panic!("NOT REAL BOARD")
			},
			None => {

			}

		}
		let mut is_fixed: [bool; AREA] = [false; AREA];
		is_fixed[iter_board.position(|&x| x != 0).unwrap()] = true; //0 is false, all other numbers are true
		//loop to get all other fixed
		loop {
			match iter_board.next() {
				None => {
					break;
				},
				Some(n) => {
					is_fixed[*n as usize] = true;
				}
			}
		}
		Game {
			board : import_board,
			fixed: is_fixed
		}
	}
	///retrive value at box
	pub fn get(&self, (x, y): Position) -> u8 {
		let n = (y * WIDTH as u8) + x;
		self.board[n as usize]
	}
	///set value at box, if changable
	pub fn set(&mut self, (x, y): Position, set: u8) {
		let n: usize = ((y * WIDTH as u8) + x) as usize;
		if !self.fixed[n] { //check if mutable
			self.board[n] = set;
		}
	}
	//TODO: terrible function WIP
	pub fn set_checked(&mut self, (x, y): Position, set: u8) -> Option<usize> {
		Game::get(self, (x, y));
		let mut taken: [u8; (3 * WIDTH)] = [0; (3 * WIDTH)];
		taken[0..WIDTH].copy_from_slice(&Game::check_line_x(self, y));
		taken[9..(2 * WIDTH)].copy_from_slice(&Game::check_line_y(self, x));
		taken[18..(3 * WIDTH)].copy_from_slice(&Game::check_square(self, (x, y)));
		taken.iter().position(|&r| r == set)
	}
	///return horizontal line
	pub fn check_line_x(&self, y: u8) -> [u8; WIDTH] {
		let line = y * WIDTH as u8;
		let mut sum: [u8; WIDTH] = [0; WIDTH];
		for x in 0..WIDTH {
			sum[x as usize] = self.board[(line + x as u8) as usize];
		}
		sum
	}
	///return virtical line
	pub fn check_line_y(&self, x: u8) -> [u8; WIDTH] {
		let mut sum: [u8; WIDTH] = [0; WIDTH];
		for y in 0..WIDTH {
			sum[y as usize] = self.board[x as usize + (WIDTH * y)];
		}
		sum
	}
	//TODO: don't use position, make get square func and use square number instead
	///return square WIP
	pub fn check_square(&self, (x, y): Position) -> [u8; 9] {
		let mut sum: [u8; 9] = [0; 9];
		let mut n = 0;
		for ny in (y - 1).min(0)..=(y + 1).max(WIDTH as u8){
			for nx in (x - 1).min(0)..=(x + 1).max(WIDTH as u8) {
				sum[n] = Game::get(&self, (nx, ny));
				n += 1;
			}
		}
		sum
	}
	///return missing values
	pub fn missing(&self, set: [u8; WIDTH]) -> Vec<u8>{
		let mut missing: Vec<u8> = Vec::new();
		for n in 1..=9 {
			match set.iter().find(|&x| x == &n) {
				Some(_i) => {
					//do nothing
				},
				None => {
					missing.push(n);
				}
			}
		}
		missing
	}

}