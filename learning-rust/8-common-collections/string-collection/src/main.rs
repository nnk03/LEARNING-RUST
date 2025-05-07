#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    string_collection();
}

fn string_collection() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let hello = String::from("नमस्ते");

    let mut s = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = "bar";

    s.push_str(s2);
    println!("s = {s}, s2 = {s2}");

    s.push('l');
    println!("s = {s}");

    let s1 = String::from("Hello");
    let s2 = String::from(" World");

    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("s3 = {s3}");
    // println!("s1 = {s1}");

    // string concatenation requires an owned `String` on the left
    // hence, the below won't work
    // let s4 = &s3 + &s2;
    //
    // println!("s4 = {s4}, s3 = {s3}");

    let f: &str = &s3;

    let ff: &&str = &f;
    println!("{}", ff);
    println!("{}", &ff);

    let ff: &str = &f;
    println!("{}", ff);
    println!("{}", &ff);

    // this won't work
    // let ff: &&&str = &f;
    // println!("{}", ff);
    // println!("{}", &ff);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{s1}");
    // println!("{s2}");
    // println!("{s3}");
    // println!("{s}");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s}");

    let s = String::from("hello");
    // let ch = &s[0];

    let hello = String::from("नमस्ते");
    // byte index 4 is not a char boundary
    // let s = &hello[0..4];
    // println!("{s}")

    // since 3 is a boundary, the below works, but generally not a good idea,
    // as it can crash our program
    // let s = &hello[0..3];
    // println!("{s}");

    for c in hello.chars() {
        println!("{c}")
    }

    for c in hello.chars() {
        println!("{c}")
    }

    for b in "Зд".bytes() {
        println!("{b}")
    }
}
