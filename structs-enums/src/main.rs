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
    // Structs
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

    // Enums & Matching
    #[derive(Debug)]
    enum Nickels {
        Liberty,
        Buffalo,
        Shield,
        Jefferson,
    }
    enum Coin {
        Penny,
        Nickel(Nickels),
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel(nickel) => {
                println!("Wow a {:?} nickel! Neat!", nickel);
                5
            },
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    value_in_cents(Coin::Penny);

    // Match all options
    let roll = 7;
    match roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {
        println!("Here, take this fancy hat someone left behind");
    }

    fn remove_fancy_hat() {
        println!("Remove your fancy hat, sir");
    }
}
