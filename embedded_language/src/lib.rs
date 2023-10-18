#[no_mangle]
pub extern "C" fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[no_mangle]
pub extern "C" fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci (n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_2() {
        let result = fibonacci(10);
        assert_eq!(result, 89);
    }
}
