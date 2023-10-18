fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result: i32 = largest_i32(&number_list);
    println!("Largest: {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['m', 'y', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);


    // let number_list = vec![34, 50, 25, 100, 65];
    // let result: i32 = largest(&number_list);
    // println!("Largest: {}", result);
    //
    // let char_list = vec!['m', 'y', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let first_point = Point {x: 1, y: 2};
    let second_point = Point {x: 1.0, y: 2.0};
    let third_point = Point {x: 1.0, y: 2};
    println!("{:?}", first_point);
    println!("{:?}", second_point);
    println!("{:?}", first_point.x);
    println!("{:?}", first_point.y);
    println!("{:?}", second_point.x);
    println!("{:?}", second_point.y);
    println!("{:?}", third_point.x);
    println!("{:?}", third_point.y);
    // third_point.x();

    let fourth_point = AnotherPoint {x: 1.0, y: 1.0};
    println!("{:?}", fourth_point.x);
    println!("{:?}", fourth_point.y);

    println!("{:?}", fourth_point.distance());
    println!("{:?}", first_point.mixup(second_point));
}

fn largest_i32(numbers: &[i32]) -> i32 {
    numbers.iter().reduce(|x, y| if x > y { x } else { y }).copied().unwrap()
}

fn largest_char(chars: &[char]) -> char {
    chars.iter().reduce(|x, y| if x > y { x } else { y }).copied().unwrap()
}

// fn largest<T>(list: &[T]) -> T {
//     list.iter().reduce(|x, y| if x > y { x } else { y }).copied().unwrap()
// }

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<X1: Clone, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(&self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x.clone(),
            y: other.y,
        }
    }
}

struct AnotherPoint<T> {
    x: T,
    y: T,
}

impl AnotherPoint<f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
