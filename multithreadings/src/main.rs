use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawn: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap();

    for i in 1..5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });
    handle.join().unwrap();
}

// It only works until main exit

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("spawn: {}", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     for i in 1..5 {
//         println!("main: {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }
