use add_one;
use rand;

fn main() {
    println!("Hello, world!");
    using_add_one();
}

fn using_add_one() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
