#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    vector_collection();
}

fn vector_collection() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3, 4, 5];

    let mut v = vec![1; 5];

    for (i, &value) in v.iter().enumerate() {
        println!("{i} : {value}");
    }

    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    for (i, &value) in v.clone().iter().enumerate() {
        println!("{i} : {value}");
    }

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("Third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("third element is {third}"),
        None => println!("there is no 3rd element"),
    }

    match third {
        Some(third) => println!("third element is {third}"),
        None => println!("there is no 3rd element"),
    }

    // let third = v[2];

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is {first}");

    for i in &v {
        println!("{i}")
    }

    let mut v = vec![100, 23, 245];
    for i in &mut v {
        *i += 50;
    }
}
