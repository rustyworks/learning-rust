macro_rules! our_macro {
    () =>  { 1 + 1};
    (owo) =>  { 2 + 2};
}

macro_rules! unless {
    ($condition:expr, $code:block) => {
        if !($condition) {
            $code
        }
    }
}

fn main() {
    println!("{}", our_macro!());
    println!("{}", our_macro!(owo));
    unless!(3 > 5, {
        println!("It is from unless macro")
    });
}
