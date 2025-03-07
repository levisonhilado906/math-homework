use rand::{Rng, SeedableRng, XorShiftRng};

fn main() {
    let mut rng = XorShiftRng::from_seed(rand::Seed::new());

    println!("Random number: {}", rng.gen_range(1..=10));
}
