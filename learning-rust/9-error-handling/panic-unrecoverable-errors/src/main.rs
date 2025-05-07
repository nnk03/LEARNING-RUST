#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    panic_unrecoverable_errors();
}

fn panic_unrecoverable_errors() {
    panic!("Program crash");
}
