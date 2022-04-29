/// This function divides two numbers.
///

pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn sub_test(){
        assert_eq!(sub(9,2), 7);
        assert_eq!(sub(6, 9), -3);
    }
    #[test]
    fn div_test(){
        assert_eq!(div(10, 2), 5);
        assert_eq!(div(6, 3), 2);
    }
    #[test]
    #[should_panic]
    fn div_panic(){
        assert_eq!(div(10, 0),0)
    }
}