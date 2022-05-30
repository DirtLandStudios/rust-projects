///input x is until what number should the sequence continue
pub fn fibonacci_seq(x: u32) -> Vec<u32>{
	let mut seq: Vec<u32> = vec![0, 1, 1];
    let mut p = 1;
	let mut c = 2;
    let mut c_temp;
    while c < x {
		seq.push(c);
        c_temp = c;
		c += p;
        p = c_temp;
	}
	seq
}