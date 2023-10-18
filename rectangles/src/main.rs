fn main() {
    // First Iteration
    // let width1 = 30;
    // let height1 = 50;

    // Second Iteration
    // let rect1 = (30, 50);

    // Third Iteration
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangles is {} square pixels.",
        // area(width1, height1)
        // area(rect1)
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
    dbg!("AAAAAAAAAAAAA");
    dbg!(&rect1);
}

// fn area(width: usize, height: usize) -> usize {
//     return width * height;
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
