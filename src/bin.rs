
fn main() {

}

struct Board([u8; (9*9)]);
impl Board {
	pub fn get(self, x: u8, y: u8) -> u8 {
		let n = (y * 9) + x;
	}
	pub fn set(&self, x: u8, y: u8, set: u8) {
		let n = (y * 9) + x;
	}
}