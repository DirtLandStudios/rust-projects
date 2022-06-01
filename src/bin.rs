
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
	}
	pub fn check_line_x(&self, y: u8) -> Vec<u8> {
		let line = y * 9;
		let mut sum: Vec<u8> = Vec::new();
		for x in 0..9 {
			sum.push(self.board[(line + x) as usize]);
		}
		sum.sort();
		sum
	}
	pub fn check_line_y(&self, x: u8) -> Vec<u8> {
		let mut sum: Vec<u8> = Vec::new();
		for y in 0..9 {
			sum.push(self.board[(x + (9 * y)) as usize]);
		}
		sum.sort();
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