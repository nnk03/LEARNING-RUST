#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    while_let_example();
}

fn while_let_example() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}
