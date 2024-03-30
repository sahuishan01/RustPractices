use std::env;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_contain(&self, other: &Rectangle)-> bool{
        self.height > other.height && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn it_works() {
        env::set_var("RUST_BACKTRACE", "full");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn panic_function(){
        panic!("panic!!!")
    }

    #[test]
    fn function_2(){
        assert_eq!(2+2, 4);
    }

    #[test]
    fn rectange_test(){
        let rec1 = Rectangle{ width: 20, height: 10};
        let rec2 = Rectangle{ width: 5, height: 15};

        assert_eq!(rec1.can_contain(&rec2), true);
        let rec3 = Rectangle{ width: 5, height: 5};
        assert_eq!(rec1.can_contain(&rec3), true);

    }
}
