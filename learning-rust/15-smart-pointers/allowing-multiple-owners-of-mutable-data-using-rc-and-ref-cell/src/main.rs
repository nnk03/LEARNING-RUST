#![allow(dead_code)]

use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

fn main() {
    println!("Hello, world!");
    example_of_rc_together_with_ref_cell();
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn example_of_rc_together_with_ref_cell() {
    let value = Rc::new(RefCell::new(5));

    // b -
    //    \
    //    a - ......
    //   /
    // c

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // value is of type Rc<RefCell<i32>>
    // *value is of type RefCell<i32>
    // the below increments by 10
    *value.borrow_mut() += 10;

    // the below doesn't increment by 10... WHY ?
    // let mut x = *value.borrow_mut();
    // x = x + 10;
    // because the type of `x` is i32, and it is getting copied ..?

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
