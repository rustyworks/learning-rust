use num_bigint::BigInt;
use num_traits::One;


#[allow(dead_code)]
fn main() {
    // println!("Fibonacci: {}", fibonacci(10));
    // println!("100 celcius = {}", celcius_to_fahrenheit(100.0));
    // println!("100 factorial: {}", factorial(10_000));
    // println!("100 factorial: {}", factorial_rayon(10_000));
    println!("100 factorial: {}", fake_factorial(154));
    // println!("100 factorial: {}", fake_factorial(274_624));
}

#[allow(dead_code)]
fn fibonacci(n: i32) -> i32 {
    match n {
        0 | 1 | 2 => 1,
        _ => { fibonacci(n - 1) + fibonacci(n - 2) },
    }
}

#[allow(dead_code)]
fn celcius_to_fahrenheit(n: f64) -> f64 {
    9.0 * n / 5.0 + 32.0
}

#[allow(dead_code)]
fn factorial(n: i32) -> BigInt {
    match n {
        1 => One::one(),
        _ => n * factorial(n - 1)
    }
}

#[allow(dead_code)]
fn fake_factorial(n: i64) -> Box<i64> {
    match n {
        1 => Box::new(1),
        _ => {
            let result = *Box::new(n) + *fake_factorial(n - 1);
            Box::new(result)
        }
    }
}

#[allow(dead_code)]
fn factorial_rayon(n: i32) -> BigInt {
    if n <= 1 {
        One::one()
    } else {
        let mid = n / 2;
        let (left, right) = rayon::join(|| factorial(mid), || factorial(n - mid));
        left * right
    }
}
