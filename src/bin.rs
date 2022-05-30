
fn main() {
	let mut n: usize = 0;
	for i in 1..=1000 {
		println!("{}", i);
		n += usize::pow(i, i.try_into().unwrap());
	}
	println!("{}", n);
/* 	let n = fibonacci_seq(40000000);
	let mut p: Vec<u32> = Vec::new();
	for i in 0..n.len() {
		if n[i] % 2 == 0 {
			p.push(n[i]);
		}
	}
	println!("{:?}", p); */
}