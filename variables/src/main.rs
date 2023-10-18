fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("{x}");
    }
    println!("{x}");

    let f = 0.1 + 0.1 + 0.1;
    println!("{f}");

    let i = 10 / 3;
    println!("{i}");

    let a: [i32; 5] = [3; 5];
    println!("{:?}", a);
    println!("{}", a[a.len() - 1]);
}
