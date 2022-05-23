mod invrs_factorial;
mod ceasar_ciper;
mod fibonacci_sequence;
pub use invrs_factorial::invrs_fact;
pub use ceasar_ciper::ceasar;
pub use ceasar_ciper::ceasar_bin;
pub use fibonacci_sequence::fibonacci_seq;

/*
mod invrs_factorial;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{}", invrs_factorial::invrs_fact(120).unwrap());
    }
}
*/
