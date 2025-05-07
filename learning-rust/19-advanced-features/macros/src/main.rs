#![allow(dead_code)]

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use macros::custom_vec;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    println!("Hello, world!");

    let v = custom_vec!(1, 2, 3, 4, 5);
    println!("{:?}", v);

    Pancakes::hello_macro();
}
