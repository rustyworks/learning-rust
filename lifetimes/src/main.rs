use std::fmt::Display;


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1.as_str(), &string2);
    println!("Result: {}", result);

    let a = 1;
    let b = 2;
    let high = highest(&a, &b);
    println!("Highest: {}", high);
    println!("a: {}, b: {}", a, b);
    println!("string1: {}, string2: {}", string1, string2);

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }
    println!("string3: {}", string3);
    // println!("string4: {}", string4);

    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence: &str = novel.split('.').next().expect("Couldn't find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("longest string is: {}", longest_with_an_announcement(string1, string2));
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn highest(x: &i32, y: &i32) -> i32 {
    if x > y {
        *x
    } else {
        *y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// struct ImportantExcerpt {
//     part: &str,
// }

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
