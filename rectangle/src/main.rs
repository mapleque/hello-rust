#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        dbg!(dbg!(self.width) * dbg!(self.height))
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
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
    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };
    dbg!(&rect1);
    dbg!(&rect2);
    println!("The ares of the rect1 is {} square pixels.", rect1.area());
    println!("The ares of the rect2 is {} square pixels.", rect2.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    let rect3 = Rectangle::square(55);
    dbg!(&rect3);
    dbg!(&rect3.can_hold(&rect1));
    dbg!(&rect3.can_hold(&rect2));
}
