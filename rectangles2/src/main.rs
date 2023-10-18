fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {}", rect1.area());

    if rect1.width() {
        println!("The width or rectangle is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(50);
    println!("The rectangle square: {:#?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
