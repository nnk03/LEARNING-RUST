#![allow(dead_code)]

use std::rc::Rc;

#[derive(Debug)]
enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    println!("Hello, world!");
    cannot_use_box_for_the_purpose();
    two_list_sharing_third_list_using_rc();
}

fn two_list_sharing_third_list_using_rc() {
    // 2 list that both share ownership of a third list
    // b -
    //    \
    //    a - ......
    //   /
    // c

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let d = Cons(4, Rc::clone(&a));
        println!("Count after creating d = {}", Rc::strong_count(&a));
    }
    println!("Count after d goes out of scope = {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    println!("Count after creating c = {}", Rc::strong_count(&a));
}

fn cannot_use_box_for_the_purpose() {
    // 2 list that both share ownership of a third list
    // b -
    //    \
    //    a - ......
    //   /
    // c

    // the below won't work
    // let a = ListBox::Cons(5, Box::new(ListBox::Cons(10, Box::new(ListBox::Nil))));
    // let b = ListBox::Cons(3, Box::new(a));
    // let c = ListBox::Cons(4, Box::new(a));
}
