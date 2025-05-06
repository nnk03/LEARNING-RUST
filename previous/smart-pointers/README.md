Given that the smart pointer pattern is a general design
pattern used frequently in Rust, this chapter won’
t cover every existing smart pointer. Many libraries have
their own smart pointers, and you can even write
your own. We’ll cover the most common
smart pointers in the standard library:

1.  Box<T> for allocating values on the heap

1.  Rc<T>, a reference counting type
    that enables multiple ownership

1.  Ref<T>
    and RefMut<T>, accessed through RefCell<T>, a type that
    enforces the borrowing rules at runtime instead of compile time

In addition, we’ll cover the interior mutability
pattern where an immutable type exposes an API for mutating
an interior value. We’ll also discuss reference
cycles: how they can leak memory and how to
prevent them.

# Box<T>

Boxes don’t have performance overhead, other than
storing their data on the heap instead of on the
stack. But they don’t have many extra
capabilities either. You’ll use them most often
in these situations:

1. When you have
   a type whose size can’t be known at
   compile time and you want to use a value of
   that type in a context that requires an exact size

1. When you have a large amount of data
   and you want to transfer ownership but ensure the data
   won’t be copied when you do so
1. When you want to own a value and you care
   only that it’s a type that implements a
   particular trait rather than being of a specific type

Rust does deref coercion when it finds types and trait implementations in three cases:

1. From &T to &U when T: Deref<Target=U>
1. From &mut T to &mut U when T: DerefMut<Target=U>
1. From &mut T to &U when T: Deref<Target=U>

# RC<T>

Note that Rc<T> is only for use
in single-threaded scenarios. When we discuss concurrency

Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules discussed in Chapter 4: multiple mutable borrows to the same place can cause data races and inconsistencies. But being able to mutate data is very useful! In the next section, we’ll discuss the interior mutability pattern and the RefCell<T> type that you can use in conjunction with an Rc<T> to work with this immutability restriction.
in Chapter 16, we’ll cover how to
do reference counting in multithreaded programs.

# RefCell<T>

Unlike Rc<T>, the RefCell<T>
type represents single ownership over the data it holds.
So, what makes RefCell<T> different from
a type like Box<T>? Recall the borrowing
rules you learned in Chapter 4:

1. At any
   given time, you can have either (but not
   both) one mutable reference or any number of immutable
   references.
1. References must always be valid.
   With references
   and Box<T>, the borrowing rules’ invariants
   are enforced at compile time. With RefCell<T> , these invariants are enforced at runtime. With references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you break these rules, your program will panic and exit.

   Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context. We’ll talk about how to get the functionality of RefCell<T> in a multithreaded program in Chapter 16.

Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
Mutating the value inside an immutable value is the interior mutability pattern. Let’s look at a situation in which interior mutability is useful and examine how it’s possible.

The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>. Both types implement Deref, so we can treat them like regular references.

# Reference cycles can leak memory

Creating reference cycles is not easily done, but it’s not impossible either. If you have RefCell<T> values that contain Rc<T> values or similar nested combinations of types with interior mutability and reference counting, you must ensure that you don’t create cycles; you can’t rely on Rust to catch them. Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and other software development practices to minimize.

Another solution for avoiding reference cycles is reorganizing your data structures so that some references express ownership and some references don’t. As a result, you can have cycles made up of some ownership relationships and some non-ownership relationships, and only the ownership relationships affect whether or not a value can be dropped. In Listing 15-25, we always want Cons variants to own their list, so reorganizing the data structure isn’t possible. Let’s look at an example using graphs made up of parent nodes and child nodes to see when non-ownership relationships are an appropriate way to prevent reference cycles.
