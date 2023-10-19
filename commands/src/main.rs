use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .arg("-alhtr")
        .output()
        .expect("Command error")
        .stdout;
    println!(
        "The output: {}",
        String::from_utf8_lossy(&output)
    );
    println!("{}", f64::from(3));
    println!("{}", "3".parse::<f64>().unwrap());
}
