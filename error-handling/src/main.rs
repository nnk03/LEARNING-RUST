use core::panic;
use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

// How to make the below code work
// i.e new function returns a Result type one is new Guess and other is Error after
// performing the check
//
// but is that possible or new function should always return the type of the structure???
/* pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, Error> {
        if value < 1 || value > 100 {
            // panic!("Guess value must be between 1 and 100, got {value}.");
            Error("HELO")
        }

        // why this is a problem
        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
} */

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // panic!("Crash and Burn");
}
