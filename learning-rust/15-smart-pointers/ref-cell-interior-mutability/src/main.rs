#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    cannot_borrow_immutable_value_mutable_normally();
}

fn cannot_borrow_immutable_value_mutable_normally() {

    // the below doesn't work
    // let x = 5;
    // let y = &mut x;
}
