#![allow(dead_code)]

use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    result_recoverable_errors();
    // let greeting_file = File::open("Hello.txt")?;
    //

    let greeting_file = File::open("Hello.txt")?;

    Ok(())
}

fn result_recoverable_errors() {
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file {:#?}", error)
    //     }
    // };

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:#?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:#?}", other_error)
    //         }
    //     },
    // };

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file : {:#?}", error);
            })
        } else {
            panic!("Problem opening the file {:#?}", error);
        }
    });

    // let greeting_file = File::open("Hello.txt").unwrap();

    // let greeting_file =
    //     File::open("Hello.txt").expect("Hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("world.txt");

    // what is the problem in the below
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     error => return error,
    // };

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // what is the problem in this
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     error => error,
    // }

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    // let mut username_file = File::open("Hello.txt")?;
    // let mut username = String::new();
    //
    // username_file.read_to_string(&mut username)?;
    //
    // Ok(username)

    // OR THE BELOW WAY
    let mut username = String::new();
    File::open("Hello.txt")?.read_to_string(&mut username)?;

    Ok(username)

    // OR THE BELOW WAY
    // fs::read_to_string("Hello.txt")
}

fn last_char_of_file_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
