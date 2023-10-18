use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores.get("Blue"));
    println!("{:?}", scores.get("Blue").copied().unwrap_or(0));
    for (key, value) in &scores {
        println!("{key} - {value}");
    }
    println!("{:?}", scores.get("Blue").unwrap());

    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores.get("Blue").unwrap());

    scores.entry(String::from("Red")).or_insert(99);
    scores.entry(String::from("Blue")).or_insert(99);
    println!("{:?}", scores.get("Blue").unwrap());
}
