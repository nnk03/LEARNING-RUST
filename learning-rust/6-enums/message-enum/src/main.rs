#![allow(dead_code)]

fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method definition goes here
        }
    }
    println!("Hello, world!");
    let m = Message::Write(String::from("hello"));
    m.call();

    let m = Message::Quit;
    if let Message::Write(s) = m {
        println!("{}", s);
    } else if let Message::Quit = m {
        println!("Message::Quit");
    }

    let x = Some(5);
    match x {
        Option::Some(num) => println!("Some {}", num),
        Option::None => println!("None"),
    }

    let no_number: Option<i32> = None;
    let equivalent_no_number = None::<i32>;
}
