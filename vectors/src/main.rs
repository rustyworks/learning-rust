// use std::any::Any;


fn main() {
    let mut v: Vec<i32> = Vec::new();
    let y = vec![1, 2, 3];

    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);
    v.push(1);
    // let x: Vec<dyn Any> = Vec::new();
    println!("{:?}", v);
    // println!("{:?}", x);
    println!("{:?}", y);
    println!("{}", y[1]);
    println!("{}", y[1]);
    let a = v[3];
    f(v[3]);
    f(v[3]);
    println!("{}", v[3]);
    println!("{}", v[3]);

    let mut v: Vec<String> = Vec::new();
    v.push(String::from("5"));
    v.push(String::from("4"));
    v.push(String::from("3"));
    v.push(String::from("2"));
    v.push(String::from("1"));
    g(&v[3]);
    println!("{:?}", v);

    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);
    v.push(1);

    let mut a: &i32 = &v.clone()[3];
    v.push(0);
    println!("{}", a);

    let mut s: String = String::from("नमस्ते");
    println!("{:?}", &s[0..6]);
    println!("{:?}", s.chars().nth(0));
    println!("{:?}", s.chars().nth(1));
    println!("{:?}", s.chars().nth(2));
    println!("{:?}", s.chars().nth(3));
    println!("{:?}", s.chars().nth(4));
    println!("{:?}", s.chars().nth(5));
    println!("{:?}", s.chars().nth(6));
}

fn f(a: i32) -> i32 {
    a
}

fn g(a: &String) -> &String {
    a
}
