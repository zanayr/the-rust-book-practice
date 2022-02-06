use std::fmt::Display;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Structs
struct Point<U, V> {
    x: U,
    y: V,
}

impl<U, V> Point<U, V> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl<U, V> Point<U, V> {
    fn mixup<X, Y>(self, other: Point<X, Y>) -> Point<U, Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Unique to 32 bit floats
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Traits
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Uses default summarize
// impl Summary for Article {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({}", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// If this was a seperate binary
// use aggregator::{ Summary, Tweet };

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Lifetimes
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

fn longest_with_an_announcment<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
 ) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest number is {}", result);

    // Structs
    let point_1 = Point { x: 5, y: 10 };
    let point_2 = Point { x: -5, y: -10 };

    println!("point_1.x = {}", point_1.x());

    let point_3 = point_1.mixup(point_2);
    println!("point_3.x = {}, point_3.y = {}", point_3.x, point_3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let sentence = "Call me Ishmael";
    println!("The first word: {}", first_word(sentence));
}
