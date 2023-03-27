#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// start an implementation block - everything in here will be associated with Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // "Class level" method - returns an instance of Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::square(25)

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}