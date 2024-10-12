fn main() {
    println!("Hello, world!");
    println!("{}", plus_one(5));

    let x = plus_one(4);

    if x < 5 {
        println!("Less than 5");
    } else if x == 5 {
        println!("Equal to 5");
    } else {
        println!("Greater than 5");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
