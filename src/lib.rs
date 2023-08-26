struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    
    fn fail_test() {
        panic!("Make this test fail");
    }
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
