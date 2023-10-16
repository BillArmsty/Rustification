struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //use the & in front of the self shorthand to indicate that this method borrows the Self instance
    // we just want to read the data in the struct, not write to it.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// Impl block is where we define functions associated with the struct
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
