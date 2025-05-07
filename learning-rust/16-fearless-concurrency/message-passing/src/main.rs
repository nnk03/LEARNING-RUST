#![allow(dead_code)]

use std::{sync::mpsc, thread, time::Duration};

fn main() {
    println!("Hello, world!");
    example_of_channels();
}

fn example_of_channels() {
    // mpsc = Multiple Producer Single Consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // println!("Val is {val}");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // thread::sleep(Duration::from_secs(2));
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {received}");

    for received in rx {
        println!("Got: {received}");
    }
}
