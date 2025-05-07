#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    println!("Hello, world!");
    // mutex_example();
    sharing_mutex_between_multiple_threads();
}

fn sharing_mutex_between_multiple_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());

    // The below also won't compile, since Rc is not safe to use within threads
    //
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result : {}", *counter.lock().unwrap());

    // the below won't compile
    //
    // let counter = Mutex::new(0);
    //
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result : {}", *counter.lock().unwrap());
}

fn mutex_example() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
