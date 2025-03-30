fn main() {
    // Example of using a loop to calculate factorials
    let num = 5;
    let factorial = 1;

    while num > 0 {
        factorial *= num;
        num -= 1;
    }

    println!("Factorial of {} is {}", num, factorial);
}
