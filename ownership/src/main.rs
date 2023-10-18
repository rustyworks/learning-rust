fn main() {
    let s1 = "Hello";
    let _x1 = s1;
    println!("{s1}");
    let s2 = String::from("Hello");
    let _x2 = s2;
    // println!("{s2}");

    let mut s3 = String::from("World");
    borrow_ownership(&mut s3);
    println!("{s3}");

    let s4: String = String::from("Abc");
    let refer: &String = &s4;
    println!("{s4}, {refer}");

    // let reference_to_nothing = dangle();
    let not_reference = not_dangle();
    println!("{:?}", not_reference.chars().rev());
}

fn borrow_ownership(x: &mut String) {
    x.push_str("!");
}

// fn dangle() -> &String {
//     let s: String = String::from("Hello");
//
//     return &s;
// }

fn not_dangle() -> String {
    let s  : String = String::from("Hello");

    return s;
}
