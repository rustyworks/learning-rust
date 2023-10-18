use std::thread;

fn main() {
    let v = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];

    thread::scope(|s| {
        let mut handles = Vec::new();
        // I want to process 4 chunks per threads.
        for chunk in v.chunks(4) {
            // I try poc for multithread here
            let handle = s.spawn(move || {
                let mut sum = 1;
                // This commented code will cause error
                for ch in chunk.iter() {
                    sum *= ch
                }
                sum
            });
            handles.push(handle);
        }
        let mut result: i32 = 0;
        for handle in handles {
            result += handle.join().unwrap();
        }
        println!("{:?}", v);
        println!("{:?}", result);
    });
}
