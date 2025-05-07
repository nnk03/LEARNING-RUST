#![allow(dead_code)]

fn main() {
    println!("Hello, world!");

    let r = Rectangle {
        width: 10,
        height: 5,
    };

    dbg!(&r);
    println!("{:#?}", r);
    println!("Area is {}", find_area_of_rectangle(&r));
    println!("Area is {}", r.area());
    println!("Area is {}", (&r).area());
    println!("Area is {}", (&(&r)).area());

    let s = Rectangle::square(5);
    println!("Area is {}", s.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn find_area_of_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
