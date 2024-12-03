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
