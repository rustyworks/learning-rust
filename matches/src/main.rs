fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let seven = plus_one(Some(7));
    let a = 5;

    if let 5 = a {
        println!("5 called");
    }
    if let 4 = a {
        println!("4 called");
    }

    println!("{:?} {:?} {:?} {:?}", five, six, none, seven);
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        // Coin::Quarter => 25,
        _ => 0,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
