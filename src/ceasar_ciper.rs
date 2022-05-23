pub fn ceasar(inp: &str, inshift: usize, back: bool) -> String {
    let input: Vec<char> = inp.to_lowercase().chars().collect();
	let shift;
    if !back {
		shift = 26 + inshift % 26;
    } else {
		shift = 26 - inshift % 26;
    }
	let language: String =
	"abcdefghijklmnopqrstuvwxyz".to_owned();
	let dictionary: Vec<char> = language.chars().collect();
    let mut output = String::new();
	for i in input {
		match dictionary.iter().position(|&r| r == i) {
			Some(mut n) => {
				n += shift;
				output.push(dictionary[n % 26]);
			},
			None => {
				//it's a space
				output.push(' ');
			}
		}

	}
    output
}

pub fn ceasar_bin(binary: Vec<u8>, shift: u8, back: bool) -> Vec<u8> {
	let mut input: Vec<u8> = binary;
	if !back {
		for i in 0..input.len() {
			input[i] = input[i].wrapping_add(shift);
		}
	} 
	else {
		for i in 0..input.len() {
			input[i] = input[i].wrapping_sub(shift);
		}
	}
	input
}