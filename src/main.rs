use std::thread;

fn main() {
    let handle1 = thread::spawn(|| {
        for n in 0..40 {
            println!("fib1({n}) => {}", fib1(n as i64));
        }
    });

    let handle2 = thread::spawn(|| {
        for n in 0..40 {
            println!("fib2({n}) => {}", fib2(n as i64));
        }
    });

    let handle3 = thread::spawn(|| {
        for n in 0..40 {
            println!("fib3({n}) => {}", fib3(n as i64));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}

// recursion and if
fn fib1(n: i64) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib1(n - 1) + fib1(n - 2)
    }
}

// recursion and match
fn fib2(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib2(n - 1) + fib2(n - 2),
    }
}

// iteration for efficiency
fn fib3(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut a: i64 = 0;
    let mut b: i64 = 1;
    let mut result: i64 = 0;

    for _ in 2..=n {
        result = a + b;
        a = b;
        b = result;
    }

    result
}
