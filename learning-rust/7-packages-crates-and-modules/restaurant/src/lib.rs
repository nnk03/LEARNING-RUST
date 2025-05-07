#![allow(dead_code)]

// use std::fmt::Result;
// use std::io::Result as IoResult;
// use std::fmt::*;
// use std::io::*;
//
// fn hello() -> Result {
//     Ok(123)
// }

mod back_of_house;
mod customer;
mod front_of_house;

fn deliver_order() {}

// now add_to_waitlist can be called as restaurant::hosting::add_to_waitlist()
// because of pub use
// else, we would have had to use restaurant::front_of_house::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // help of `use` keyword
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("Blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
