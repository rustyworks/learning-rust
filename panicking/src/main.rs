use std::fs::File;
use std::io::ErrorKind;


fn main() {
    // panic!("Burn");
    let v = vec![1, 2, 3];

    // v[99];
    let greeting_file_result = File::open("hello.txt");
    // let greeting_file_result = File::open("src/main.rs");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error)=> match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            },
        },
    };
    let greeting_file = File::open("hello_2.txt")
        .expect("hello_2.txt should be included in this project");
    println!("{:?}", greeting_file);
    println!("End");
}
