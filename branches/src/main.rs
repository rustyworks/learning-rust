fn main() {
    let number: i32 = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("x: {}", x);
}
