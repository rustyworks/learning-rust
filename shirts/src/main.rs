use std::thread;

fn main() {
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);
    //
    // thread::spawn(|| println!("From thread: {:?}", list))
    //     .join()
    //     .unwrap();

    // let store = Inventory {
    //     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    // };
    // let user_pref1 = Some(ShirtColor::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!("The user with preference: {:?} gets {:?}", user_pref1, giveaway1);
    //
    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!("The user with preference: {:?} gets {:?}", user_pref2, giveaway2);

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    list.sort_by_key(|x| x.width);
    println!("{:?}", list);

    let vec1 = vec![1, 2, 3];
    let iter_vec1 = vec1.iter();
    println!("{:?}", iter_vec1);

    let vec2 = vec![Some(1), Some(2), None, Some(4)];
    let iter_vec2 = vec2.iter();
    for i in iter_vec2 {
        println!("{:?}", i);
    }
    println!("{:?}", vec2.iter().map(|x| x));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor { 
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
