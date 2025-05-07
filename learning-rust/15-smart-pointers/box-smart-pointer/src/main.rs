#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    box_pointer_example();
    example_of_drop();
}

// Cons List
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);

        // is the below fine ?
        // std::mem::drop(self);
    }
}

use std::ops::Deref;

use crate::List::{Cons, Nil};

fn example_of_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let e = CustomSmartPointer {
        data: String::from("This has to be dropped early"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    // early drop
    std::mem::drop(e);
    println!("Dropped before scope ends");
}

fn box_pointer_example() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // behind the scenes of assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    // deref coercion
    hello(&m);

    // If Rust did not have deref coercion
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
