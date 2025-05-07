#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    iterator_examples();
}

fn iterator_examples() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got : {val}");
    }

    // this throws compiler error
    // assert_eq!(v1_iter.next(), None);

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1_iter = v1.iter();
    let total: i64 = v1_iter.sum();
    // this doesn't work, when used together with the above two lines, WHY ?
    // let v1_iter = v1.iter();
    // let total: i32 = v1_iter.sum();
    //

    // Iterator Adapters
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("v2 = {:?}", v2);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    // style : String
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![Shoe { size: 10 }, Shoe { size: 13 }, Shoe { size: 10 }];
    let in_my_size = shoes_in_size(shoes, 10);
    assert_eq!(in_my_size, vec![Shoe { size: 10 }, Shoe { size: 10 }]);
}
