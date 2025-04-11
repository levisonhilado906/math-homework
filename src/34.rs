fn main() {
    let mut n = 5;
    while n > 0 {
        println!("{}", n);
        if n % 2 == 1 {
            break;
        }
        n -= 1;
    }
}
