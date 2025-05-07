#![allow(dead_code)]

type Kilometers = i32;

fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // bar();
}

// fn generic<T: Sized>(t: T) {}

// fn generic<T: ?Sized>(t: &T) {}

fn bar() -> ! {
    panic!("HELLO WORLD");
}
