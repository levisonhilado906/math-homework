use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen_range(1..=10);
    println!("Random number: {}", x);
}
