

fn main() {

}

type Position = (u8, u8);

const WIDTH: u8 = 9;
const HEIGHT: u8 = 9;
const AREA: usize = (WIDTH * HEIGHT) as usize;

struct Game {
	board : [u8; AREA]
}

impl Game {
	pub fn new() -> Game {
		Game {board : [0; AREA]}
	}

	pub fn get(&self, (x, y): Position) -> u8 {
		let n = (y * WIDTH) + x;
		self.board[n as usize]
	}

	pub fn set(&mut self, (x, y): Position, set: u8) {
		let n = (y * WIDTH) + x;
		self.board[n as usize] = set;
	}

	pub fn set_checked(&mut self, (x, y): Position, set: u8) -> Option<usize> {
		Game::get(self, (x, y));
		let mut taken: [u8; (3 * 9)] = [0; (3 * 9)];
		taken[0..9].copy_from_slice(&Game::check_line_x(self, y));
		taken[9..(2 * 9)].copy_from_slice(&Game::check_line_y(self, x));
		//taken[18..(3 * 9)].copy_from_slice(&Game::check_square(self, x, y));
		taken.iter().position(|&r| r == set)
	}

	pub fn check_line_x(&self, y: u8) -> [u8; WIDTH as usize] {
		let line = y * WIDTH;
		let mut sum: [u8; WIDTH as usize] = [0; WIDTH as usize];
		for x in 0..WIDTH {
			sum[x as usize] = self.board[(line + x as u8) as usize];
		}
		//sum.sort();
		sum
	}

	pub fn check_line_y(&self, x: u8) -> [u8; HEIGHT as usize] {
		let mut sum: [u8; HEIGHT as usize] = [0; HEIGHT as usize];
		for y in 0..HEIGHT {
			sum[y as usize] = self.board[(x + (WIDTH * y as u8)) as usize];
		}
		//sum.sort();
		sum
	}

	pub fn check_square(&self, (x, y): Position) -> [u8; 9] {
		let mut sum: [u8; 9] = [0; 9];
		let mut n = 0;
		for nx in (x - 1).min(0)..=(x + 1).max(WIDTH) {
			for ny in (y - 1).min(0)..=(y + 1).max(HEIGHT) {
				sum[n] = Game::get(&self, (nx, ny));
				n += 1;
			}
		}
		sum
	}
}