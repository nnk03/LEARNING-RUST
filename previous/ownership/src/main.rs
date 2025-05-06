fn main() {
    let s = "Hello World";

    let mut s = String::from("Hello");
    s.push_str(", World");

    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = s1;
    // s1 moved to s2
    // s1 no longer valid ?

    // the below results in error
    // println!("{s1}");

    let mut s = String::from("Hello");

    let r1 = &mut s;
    let r2 = &mut s;

    // println!("{r1} {r2}");

    let mut s = String::from("Hello World");

    let word = first_word(&s);

    s.clear();

    // Uncommenting the below line results in an error
    // println!("The first word is {word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
