use rand::Rng;
use std::thread;
use std::time::Duration;
use std::io;
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let randomize = rng.gen_range(0..3);
        let c: char = char::from_u32(9585 + randomize).unwrap();
        thread::sleep(Duration::from_millis(1));
        print!("{}", c);
        let _ = io::stdout().flush();
    }
    println!()
}
