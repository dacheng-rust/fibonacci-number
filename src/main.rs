fn main() {
    for n in 0..20 {
        println!("{n:2},{:5}", f(n))
    }
}

fn f(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        f(n - 1) + f(n - 2)
    }
}
