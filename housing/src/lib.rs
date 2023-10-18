mod front_of_housing;

use crate::front_of_housing::hosting;


pub fn eat() {
    hosting::add_to_waiting();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
