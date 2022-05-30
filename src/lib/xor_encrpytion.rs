//https://projecteuler.net/problem=59
pub fn xor_encrpyt(mut input: Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    for i in 0..input.len() {
		input[i] ^= key[i % key.len()];
	}
	input
}