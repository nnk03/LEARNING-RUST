#![allow(dead_code)]

use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary {
    // fn summarize(&self) -> String;

    // default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{} : {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters
//
// impl Trait Syntax
//
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news, {}", item.summarize());
// }

// Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news, {}", item.summarize());
}

// here both item1 and item2 can be different types as long as they implement the trait Summary
pub fn notify_maybe_different_types(item1: &impl Summary, item2: &impl Summary) {
    let s = format!("{} {}", item1.summarize(), item2.summarize());
    println!("{s}");
}

// if we want to force both of them to be the same Type, we must use a trait bound syntax
pub fn notify_two_same_types<T: Summary>(item1: &T, item2: &T) {
    let s = format!("{} {}", item1.summarize(), item2.summarize());
    println!("{s}");
}

// for specifying multiple trait bounds
// pub fn notify_with_display(item: &(impl Summary + Display)) {
//     println!("{} {}", item, item.summarize())
// }

pub fn notify_with_display<T: Summary + Display>(item: &T) {
    println!("{} {}", item, item.summarize())
}

// with where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("t = {}", t.clone());
    println!("u = {:?}", u.clone());
    0
}

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// the below is an error
// fn returns_summarizable_alternate<T>() -> T
// where T : impl Summary
// {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }

// the below also is an error, will be covered later
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
