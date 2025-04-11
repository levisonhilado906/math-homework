use std::cmp;

fn main() {
    let mut numbers = vec![10, 20, 30, 40, 50];
    
    println!("Numbers: {:?}", numbers);
    // Find and print the largest number in the vector
    let max_num = *numbers.iter().max_by(|a, b| a.cmp(b)).unwrap();
    println!("Largest number: {}", max_num);
}
