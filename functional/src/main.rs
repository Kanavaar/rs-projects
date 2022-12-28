fn main() {
    println!("{}", (1..11).fold(0, |a, b| a + b));
    let mut sum = 0;
    for i in 1..11 {
        sum = sum + i;
    }
    println!("{}", sum);
}
