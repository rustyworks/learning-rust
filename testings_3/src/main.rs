use std::thread;
// use std::sync::{Arc, Mutex};

fn main() {
    let mut a = 1;
    let b = 1;
    println!("a: {}", a);
    println!("b: {}", b);
    {
        a = 2;
        let b = 2;
        println!("a: {}", a);
        println!("b: {}", b);
    }
    println!("a: {}", a);
    println!("b: {}", b);

    let t = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
    // println!("{:?}", t);  // this will be error
    println!("{}", t.13);

    let arr: [i32; 100] = [0; 100];
    println!("{:?}", arr);


    let mut handles = Vec::new();
    for i in 1..20 {
        let handle = thread::spawn(move || {
            i * 2
        });
        handles.push(handle);
    }
    let mut result = 0;
    for handle in handles {
        result += handle.join().unwrap();
    }
    println!("Result: {}", result);
    let result_2: i32 = (1..20).map(|x| x * 2).sum();
    println!("Result2: {}", result_2);

    let xxx: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let result_3: i32 = xxx.iter().filter(|&x| x % 2 == 0).map(|&x| x * 2).sum();
    println!("Result3: {}", result_3);

    let v = vec![6, 7, 8];
    let total: i32 = v.into_iter().map(|x| x * 3).filter(|&x| x % 2 == 0).sum();
    println!("{}", total);

    // let mut handles = Vec::new();
    // let v = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 ];
    // for chunk in v.chunks(4) {
    //     let handle = thread::spawn(move || {
    //         let mut sum = 1;
    //         for ch in chunk {
    //             sum *= ch
    //         }
    //         let sum = sum;
    //         sum
    //     });
    //     handles.push(handle);
    // }
    // let mut rez: i32 = 0;
    // for handle in handles {
    //     rez += handle.join().unwrap();
    // }
    // println!("{:?}", v);

    let mut handles = Vec::new();
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    for chunk in v.chunks(4) {
        let handle = thread::spawn(move || {
            let mut sum = 1;
            for ch in chunk {
                sum *= ch;
            }
            sum
        });
        handles.push(handle);
    }
    let mut rez: i32 = 0;
    for handle in handles {
        rez += handle.join().unwrap();
    }
    println!("{:?}", v);

    // let mut handles = Vec::new();
    // let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    // 
    // let v_arc = Arc::new(Mutex::new(v));
    //
    // for chunk in v_arc.lock().unwrap().chunks(4) {
    //     let v_clone = Arc::clone(&v_arc);
    //     let handle = thread::spawn(move || {
    //         let mut sum = 1;
    //         for ch in chunk {
    //             sum *= ch;
    //         }
    //         sum
    //     });
    //     handles.push(handle);
    // }
    //
    // let mut rez: i32 = 1; // Initialize with 1, not 0
    // for handle in handles {
    //     rez *= handle.join().unwrap(); // Multiply results, not add
    // }
    // println!("Result: {}", rez);
}
