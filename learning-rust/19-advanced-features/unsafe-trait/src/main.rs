#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {}
