#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    function_generics();
    struct_generics();
    method_generics();
}

// generics in struct definitions
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointDifferent<T, U> {
    x: T,
    y: U,
}

// generics in methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> PointDifferent<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointDifferent<X2, Y2>) -> PointDifferent<X1, Y2> {
        PointDifferent {
            x: self.x,
            y: other.y,
        }
    }
}

fn method_generics() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointDifferent { x: 5, y: 10.4 };
    let p2 = PointDifferent { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3 = {:#?}", p3);
}

// generics in Enum definitions
#[derive(Debug)]
enum CustomOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum CustomResult<T, E> {
    Ok(T),
    Err(E),
}

fn struct_generics() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // let wont_work = Point { x: 5, y: 4.0 };
    let both_integer_point = PointDifferent { x: 5, y: 10 };
    let both_float_point = PointDifferent { x: 5.0, y: 10.0 };
    let integer_and_float_point = PointDifferent { x: 1, y: 10.0 };
}

fn function_generics() {
    // let number_list = vec![1, 2, 3, 4, 5, 1230];
    // let result = largest(&number_list);
    // println!("The largest number is {result}");
    //
    // let char_list = vec!['a', 'e', 'i', 'o', 'u'];
    //
    // let result = largest(&char_list);
    // println!("The largest char is {result}");
}

// generics in function definitions
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//
//     largest
// }
