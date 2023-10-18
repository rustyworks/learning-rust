// fn main() {
//     let mut counter: i32 = 0;
//     let result: i32 = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//
//     println!("The result: {result}");
// }

fn main() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("Count: {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count +=1;
    }

    println!("End count: {count}");

    for number in (1..=4).rev() {
        println!("{number}");
    }
}
