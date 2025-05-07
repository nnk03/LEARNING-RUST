#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    generic_lifetimes_in_function_example();
    struct_lifetime();
    static_lifetimes();
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn generic_lifetimes_in_function_example() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // the below will work since "xyz" is a string literal have a 'static lifetime which
    // means they live for the entire duration of the program
    //
    // let s1 = String::from("ab");
    // let result2;
    // {
    //     let s2 = "xyz";
    //     result2 = longest(s1.as_str(), s2);
    // }
    //
    // println!("The longest string is {result2}");

    // this will throw an compiler error as expected
    // let s1 = String::from("ab");
    // let result2;
    // {
    //     let s2 = String::from("xyz");
    //     result2 = longest(s1.as_str(), s2.as_str());
    // }
    //
    // println!("The longest string is {result2}");
}

fn static_lifetimes() {
    let s: &'static str = "I have a static lifetime.";
}

// lifetime annotation in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention Please : {announcement}");
        self.part
    }
}

fn struct_lifetime() {
    let novel = String::from("Hello World. Hello world");

    let first_sentence = novel.split('.').next().expect("Count not find a .");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// lifetime elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// generic lifetimes in functions
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn invalid_longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// fn invalid_reference() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     println!("r : {r}")
// }

//
// fn invalid_reference() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {r}");   //          |
// }                         // ---------+
