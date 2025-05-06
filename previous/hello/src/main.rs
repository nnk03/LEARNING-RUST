fn main(){
  let x = no_dangle();
  println!("{}", x);
}
fn no_dangle() -> String {
  let s = String::from("HELLO");
  s
}


// fn main() {
//   // let mut x = 5;
//   // println!("{x}");
//   // x = 3;
//   // println!("{x}");
//   // let a : [i32; 5] = [1, 2, 3, 4, 5];
//   // // println!("The 0th element in array a is { a[0] }");
//   // // the above is showing error ??
//   // let first = a[0];
//   // println!("{first}");
//   //
//
//   // let x = 5;
//   // let mut x = 5;
//
//   // let mut x = 5;
//   // let x = 5;
//
//   // let x = 5;
//   // // if (x % 5 == 0) {
//   // // the above throws error since parentheses is used
//   // if x % 5 == 0 {
//   //   println!("value is divisible by 5");
//   // }
//   // else {
//   //   println!("value is not divisible by 5");
//   // }
//   //
//   //
//
//   // let mut s = String::from("hello");
//   //
//   //   let r1 = &s; // no problem
//   //   let r2 = &s; // no problem
//   //   let r3 = &mut s; // BIG PROBLEM
//   //
//   //   println!("{}, {}, and {}", r1, r2, r3);
//   let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//   println!("{}", r1);
//     // variables r1 and r2 will not be used after this point
//
//     let r3 = &mut s; // no problem
//     // println!("{}", r3);
//     // println!("{} {} {}", r1, r2, r3);
//
//   // println!("{}", r1);
//
//
//
//
//
//
// }
