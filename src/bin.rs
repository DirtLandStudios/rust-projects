
fn main() {

}

struct Game {
	board : [u8; (9*9)]
}
impl Game {
	pub fn get(&self, x: u8, y: u8) -> u8 {
		let n = (y * 9) + x;
		self.board[n as usize]
	}
	pub fn set(&mut self, x: u8, y: u8, set: u8) {
		let n = (y * 9) + x;
		self.board[n as usize] = set;
	}
	pub fn set_checked(&mut self, x: u8, y: u8, set: u8) {
		Game::get(self, x, y);
		let mut taken: [u8; (3 * 9)] = [0; (3 * 9)];
		taken[0..9].copy_from_slice(&Game::check_line_x(self, y));
		taken[9..(2 * 9)].copy_from_slice(&Game::check_line_y(self, x));
		//taken[18..(3 * 9)].copy_from_slice(&Game::check_square(self, x, y));
		match taken.iter().position(|&r| r == set) {
			Some(n) => {
				
			},
			None => {

			}

		}
	}
	pub fn check_line_x(&self, y: u8) -> [u8; 9] {
		let line = y * 9;
		let mut sum: [u8; 9] = [0; 9];
		for x in 0..9 {
			sum[x] = self.board[(line + x as u8) as usize];
		}
		//sum.sort();
		sum
	}
	pub fn check_line_y(&self, x: u8) -> [u8; 9] {
		let mut sum: [u8; 9] = [0; 9];
		for y in 0..9 {
			sum[y] = self.board[(x + (9 * y as u8)) as usize];
		}
		//sum.sort();
		sum
	}
/* 	pub fn check_square(&self) -> Vec<u8> {
		let mut sum: Vec<u8> = Vec::new();
		for x in 0..3 {
			for y in 0..3 {
				sum.push(Game::get(self, x, y));
			}
		}
		sum.sort();
		sum
	} */
}