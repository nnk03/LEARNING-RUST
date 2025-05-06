# Ownership Rules

1. Each value in Rust has an owner
1. There can only be one owner at a time
1. When the owner goes out of scope, the values will be dropped

If a type implements the `Copy` trait, variables that use it do not move,
but rather are trivially copied, making them still valid after
assignment to another variable.

Rust won't let us annotate a type with `Copy` if the type, or any of
its parts, has implemented the `Drop` trait.

So, what types implement the Copy trait? You can check the documentation for the
given type to be sure, but as a general rule, any group of simple scalar values can
implement Copy, and nothing that requires allocation or is some form of resource can implement Copy

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we tried to use s after the call to takes_ownership, Rust
would throw a compile-time error. These static checks protect us from mistakes

Rust has a feature for using a value without transferring ownership, called references.
