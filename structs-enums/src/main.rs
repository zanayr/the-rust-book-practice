#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> String {
        if self.width > other.width && self.height > other.height {
            String::from("Yes")
        } else {
            String::from("No")
        }
    }
    fn square(side: u32) -> Rectangle {
        Rectangle {
            height: side,
            width: side,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        height: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        height: 45,
        width: 60,
    };

    println!("The area of `rect1` is {}", rect1.area());
    println!("Can rect1 hold rect2? {}!", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}!", rect1.can_hold(&rect3));
    println!("What about a 25 x 25 square? {}!", rect1.can_hold(&Rectangle::square(25)));
}
