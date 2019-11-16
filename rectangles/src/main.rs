#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn new_rectangle(new_length: u32, new_width: u32) -> Rectangle {
        Rectangle {length: new_length,width: new_width}
    }
}

fn main() {
    let rectangle1 = Rectangle::new_rectangle(40, 50);

    println!("rect1 is {:#?}",rectangle1);

    println!("The area of the rectangle is {} square pixels", rectangle1.area());
}



