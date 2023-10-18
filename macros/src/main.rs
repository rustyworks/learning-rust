macro_rules! our_macro {
    () =>  { 1 + 1};
    (owo) =>  { 2 + 2};
}

// macro_rules! operation {
    // ($e1: expr + $e2: expr) => { $e1 + $e2 };
    // ($e1: expr - $e2: expr) => { $e1 - $e2 };
    // ($e1: expr * $e2: expr) => { $e1 * $e2 };
    // ($e1: expr / $e2: expr) => { $e1 / $e2 };
// }

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
    // println!("{}", our_macro!(hex hex));
    // println!("operation!(5 + 2) {}", operation!(5+2));
    // println!("operation!(5 - 2) {}", operation!(5-2));
    // println!("operation!(5 * 2) {}", operation!(5*2));
    // println!("operation!(5 / 2) {}", operation!(5/2));
    unless!(3 > 5, {
        println!("AAAA")
    });
    testing();
    println!("a")
}

fn testing() {
    todo!("Fill this with something");
}
