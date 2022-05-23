
pub fn invrs_fact(input: usize) -> Option<usize> {
    match input {
		0 => None, //someone messed up
        1 => Some(1), //or 0
		_=> {
			let mut i = input;
			let mut n = 1;
			while i > 1 {
				n += 1;
				i = i/n;
			}
			if n % i == 0 {
				//println!("{}", n);
				Some(n)
			} 
			else {
				//println!("u messed up {} {}", n, i);
				None
			}
			                
        }
	}
}