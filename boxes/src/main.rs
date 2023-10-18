use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {:?}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    let x = 5;
    let y = &x;
    let z = Box::new(5);
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let z = MyBox::new(5);
    println!("{:?}", z);
    println!("{:?}", *z);
    println!("{:?}", z.deref());
    println!("{:?}", *(z.deref()));

    let xx: &str = "abc";
    println!("{}", xx);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    let m = Box::new(String::from("Rust"));
    hello(&m);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
