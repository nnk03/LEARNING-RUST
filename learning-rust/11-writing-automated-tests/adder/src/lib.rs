#![allow(dead_code)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
}

// private function
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    // #[test]
    // #[ignore]
    // fn maybe_expensive_test() {
    //     // code that takes an hour to run
    // }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_and_returns_result() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("add_two(2) does not equal 4"))
        }
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    // testing private function
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
