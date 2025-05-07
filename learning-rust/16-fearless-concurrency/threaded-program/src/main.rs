#![allow(dead_code)]

use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
    // thread_program();
    move_closure_with_thread();
}

fn move_closure_with_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Hera's a vector : {:?}", v);
    });

    handle.join().unwrap();
}

fn thread_program() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from thread_program from the main thread!?");
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();
}
