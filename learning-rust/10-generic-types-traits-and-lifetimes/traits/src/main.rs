#![allow(dead_code)]

mod aggregator;

use aggregator::{Summary, Tweet};

fn main() {
    println!("Hello, world!");
    trait_examples();
}

fn trait_examples() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet : {}", tweet.summarize());
}
