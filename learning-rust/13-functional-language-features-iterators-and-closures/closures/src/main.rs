#![allow(dead_code)]

use std::thread;

fn main() {
    println!("Hello, world!");
    closure_examples();
}

fn closure_examples() {
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        // use thread sleep
        num
    };

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // the below two shows error
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;

    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));

    // this won't compile since example_closure since we haven't added type annotations
    // let n = example_closure(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure : {:?}", list);

    let only_borrows = || println!("From Closure {:?}", list);

    println!("Before calling closure : {:?}", list);
    only_borrows();
    println!("After calling closure : {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure : {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // the below would be error
    // println!("Before calling closure : {:?}", list);
    borrows_mutably();
    borrows_mutably();
    println!("After calling closure : {:?}", list);
    // the below would be error
    // borrows_mutably();

    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    thread::spawn(move || {
        println!("From thread {:?}", list);
    })
    .join()
    .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 7,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| r.width);

    println!("{:#?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 7,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // let mut sort_operations = vec![];
    let value = String::from("by key called");

    // this won't work
    //
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    // this works
    //
    // list.sort_by_key(|r| {
    //     sort_operations.push(value.clone());
    //     r.width
    // });

    // this also works
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}", list);
    // println!("{:?}", sort_operations);
    println!("Number of times closure called {num_sort_operations}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
