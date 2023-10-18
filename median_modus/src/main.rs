fn main() {
    let array: Vec<i32> = vec![1, 2, 3, 4, 5, 4, 3, 2, 1];
    println!("{:?}", mode(&array));
}

fn median(a: Vec<i32>) -> f64 {
    1.0
}

fn mode(a: &Vec<i32>) -> i32 {
    let mut max: Option<i32> = None;
    for i in a {
        max = match max {
            None => Some(*i),
            Some(n) => {
                if i > &n {
                    Some(*i)
                } else {
                    Some(n)
                }
            }
        }
    }
    return max.unwrap();
}
