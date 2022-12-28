fn fib(n: i64) -> i64 {
    if n == 1 || n == 0 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    let fib_res = fib(16);
    println!("{}", fib_res);
}
