fn main() {
    let x: String = String::from("aaa");
    let y: i32 = 42;
    let aa = 1;
    let bb = aa;
    aaa(&x);
    bbb(y);
    println!("{}", x);
    println!("{}", y);
    println!("{}", aa);
    println!("{}", bb);

    let result = res(2).unwrap_or_else(|err| {
        println!("{}", err);
        String::from("cok")
    });
    println!("{}", result);
    let result = res(1).unwrap_or_else(|err| {
        println!("{}", err);
        String::from("cok")
    });
    println!("{}", result);
    let result = res(1).unwrap_or_default();
    println!("{}", result);

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3 = vec1.iter().zip(vec2);
    println!("{:?}", vec3);
    for x in vec3 {
        println!("{:?}", x);
    }
}

fn aaa(n: &String) -> &String {
    println!("aaaa");
    n
}

fn bbb(n: i32) -> i32 {
    println!("bbbb");
    n
}

fn res(n: i32) -> Result<String, &'static str> {
    if n % 2 == 0 {
        Ok(String::from("Mantap"))
    } else {
        Err("Gagal cok")
    }
}
