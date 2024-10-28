fn main() {
    let mut s = String::from("Hello world");

    s += "hello";

    println!("{s}");

    s.push_str("World");
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // s2 is still valid and can be used later
    println!("{s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // here all s1, s2, s3 are valid since format doesn't take
                                       // ownership of nay parameters

    println!("Hello, world!");
}
