use std::error::Error;

fn main() {
    let _ = aaa();
    let mut n: String = String::from("hello");
    xxx(&mut n);
    println!("{}", n);
    yyy(&mut n);
    println!("{}", n);
    zzz(&mut n);
    println!("{}", n);
}

fn aaa() -> Result<(), Box<dyn Error>> {
    let mut vec: Vec<String> = Vec::new();
    for _ in 10..40 {
        let gg = match Ok(String::from("abc")) {
            Ok(v) => v,
            Err(e) => { return Err(e) }
        };
        vec.push(gg);
    }
    println!("{:?}", vec);
    Ok(())
}

fn xxx(x: &mut String) -> &mut String {
    x.push_str("x1");
    x
}

fn yyy(y: &mut String) -> &mut String {
    y.push_str("y1");
    y
}

fn zzz(z: &mut String) -> &mut String {
    z.push_str("z1");
    z
}
