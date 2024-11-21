Closures are typically short and relevant only within a narrow
context rather than in any arbitrary scenario. Within these
limited contexts, the compiler can infer the types of
the parameters and the return type, similar to how
it’s able to infer the types of most
variables (there are rare cases where the compiler needs
closure type annotations too)
.

For closure definitions, the compiler will infer one concrete
type for each of their parameters and for their return
value. For instance, Listing 13-3 shows
the definition of a short closure that just returns the
value it receives as a parameter. This closure isn
’t very useful except for the purposes of this
example. Note that we haven’t added any
type annotations to the definition. Because there are no
type annotations, we can call the closure with any
type, which we’ve done here with String
the first time. If we then try to call example_closure with an integer, we’ll get an error.

```rust

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
```

The compiler gives us this error

```sh

$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^- help: try using a conversion method: `.to_string()`
  |             |               |
  |             |               expected `String`, found integer
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
 --> src/main.rs:4:29
  |
4 |     let s = example_closure(String::from("hello"));
  |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> src/main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |                            ^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` (bin "closure-example") due to 1 previous error
```

Once a closure has captured a reference or captured ownership
of a value from the environment where the closure is
defined (thus affecting what, if anything, is
moved into the closure), the code in the body
of the closure defines what happens to the references or
values when the closure is evaluated later (thus affecting
what, if anything, is moved out of the
closure). A closure body can do any of the
following: move a captured value out of the closure
, mutate the captured value, neither move nor mutate
the value, or capture nothing from the environment to begin with.

The way a closure captures and handles values from the
environment affects which traits the closure implements, and traits
are how functions and structs can specify what kinds of
closures they can use. Closures will automatically implement one
, two, or all three of these Fn traits
, in an additive fashion, depending on how the
closure’s body handles the values:

1. FnOnce applies to closures that can be called once
   . All closures implement at least this trait, because
   all closures can be called. A closure that moves
   captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
1. FnMut applies to closures that don’t move captured
   values out of their body, but that might mutate
   the captured values. These closures can be called more
   than once.
1. Fn applies to closures that
   don’t move captured values out of their body
   and that don’t mutate captured values, as
   well as closures that capture nothing from their environment.
   These closures can be called more than once without mutating
   their environment, which is important in cases such as
   calling a closure multiple times concurrently.
