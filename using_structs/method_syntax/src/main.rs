#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // order of method definitions does not matter
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        rectangle.area() <= self.area()
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated function definition
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let rect4 = Rectangle { width: 30, height: 50 };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

    // associated function invocation
    let square = Rectangle::square(5);
    println!("{:?}", square.area());
}
