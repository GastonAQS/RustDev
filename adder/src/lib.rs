pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value must be between 1 and 100")
        }
        Guess{
            value
        }
    }
}

pub fn adder(num: u32) -> u32 {
    num+2
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller(){
        let larger: Rectangle = Rectangle {length: 4, width: 9};
        let smaller: Rectangle = Rectangle{length: 5, width: 10};

        assert!(!larger.can_hold(&smaller));
    }
    #[test]
    #[should_panic(expected = "Value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
