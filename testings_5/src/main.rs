fn main() {
    let a = "abc";
    let c: String;
    {
        let b = "def";
        c = hax(a, b);
    }
    println!("{}", c);

    let a = String::from("abc");
    let b = |x| {println!("x: {}", x); x};
    let c = b(a);
    println!("{}", c);
    let a = String::from("abc");
    let c = bo(a);
    // let c = bo(a.clone());
    println!("{}", c);

    let f = |x| {x % 2 == 0};
    let result = is_odd(f, 10);
    println!("{}", result);

    let result = is_odd(f, 11);
    println!("{}", result);

    let x = vec![1, 2, 3, 4];
    let mut y = x.iter();
    println!("{:?}", y.next());

    let any_even = x.iter().any(|x| x % 2 == 0);
    println!("{}", any_even);

    let all_even = x.iter().all(|x| x % 2 == 0);
    println!("{}", any_even);

    let find_even = x.iter().find(|&&x| x % 2 == 0);
    println!("{:?}", find_even);

    let filter_even = x.iter().filter(|&x| x % 2 == 0);
    println!("{:?}", filter_even);

    let position_even = x.iter().position(|&x| x % 2 == 0);
    println!("{:?}", position_even);

    let max = x.iter().max();
    println!("{:?}", max);

    let rev = x.iter().rev();
    println!("{:?}", rev);
 }

// fn hax<'a, 'b>(a: &'a str, b: &'b str) -> &'b str {
//     b
// }

fn hax(a: &str, b: &str) -> String {
    let mut c = String::from(a);
    c.push_str(b);
    c
}

fn bo(a: String) -> String {
    println!("a: {}", a);
    a
}

fn is_odd<F: Fn(i32) ->bool>(f: F, n: i32) -> bool {
    f(n)
}
