#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    println!("Hello, world!");

    let answer = do_twice(add_one, 5);
    println!("answer = {answer}");

    let list_of_numbers = vec![1, 2, 3];

    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);

    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);

    let a_one = returns_closure();
    println!("a_one(5) = {}", a_one(5));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// the below won't compile
//
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
