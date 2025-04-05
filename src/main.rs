fn main() {
    for n in 0..20 {
        println!("fib({n}) => {}", fib(n))
    }
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
