#![allow(dead_code)]

enum Message {
    Hello { id: i32 },
}

fn main() {
    // println!("Hello, world!");

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => println!(
            "Found an id in another range, (can't use the id since it is not saved in a variable)"
        ),
        Message::Hello { id } => println!("Some other id : {id}"),
    }
}
