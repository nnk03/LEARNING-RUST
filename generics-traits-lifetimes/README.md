Other crates that depend on the aggregator crate can also
bring the Summary trait into scope to implement Summary on
their own types. One restriction to note is that
we can implement a trait on a type only if
either the trait or the type, or both,
are local to our crate. For example, we
can implement standard library traits like Display on a custom
type like Tweet as part of our aggregator crate functionality
because the type Tweet is local to our aggregator crate
. We can also implement Summary on Vec<T
in our aggregator crate because the trait Summary is local to our aggregator crate.

But we can’t implement external traits on external
types. For example, we can’t implement
the Display trait on Vec<T> within our
aggregator crate because Display and Vec<T> are
both defined in the standard library and aren’t
local to our aggregator crate. This restriction is part
of a property called coherence, and more specifically the
orphan rule, so named because the parent type is
not present. This rule ensures that other people’
s code can’t break your code and vice
versa. Without the rule, two crates could implement
the same trait for the same type, and Rust
wouldn’t know which implementation to use.

However, you can only use impl Trait if you’re
returning a single type. For example,
this code that returns either a NewsArticle or a Tweet
with the return type specified as impl Summary wouldn’t work:

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

Returning either a NewsArticle or a Tweet isn’t
allowed due to restrictions around how the impl Trait syntax
is implemented in the compiler. We’ll cover
how to write a function with this behavior in the
“Using Trait Objects That Allow for Values of Different
Types” section of Chapter 17.

We can also conditionally implement a trait for any type
that implements another trait. Implementations of a trait on
any type that satisfies the trait bounds are called blanket
implementations and are used extensively in the Rust standard library
. For example, the standard library implements the ToString
trait on any type that implements the Display trait.
The impl block in the standard library looks similar to
this code:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```sh

$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

The help text reveals that the return type needs a
generic lifetime parameter on it because Rust can’t
tell whether the reference being returned refers to x or
y. Actually, we don’t know either
, because the if block in the body of this
function returns a reference to x and the else block
returns a reference to y!

When we’
re defining this function, we don’t know
the concrete values that will be passed into this function
, so we don’t know whether the if
case or the else case will execute. We also
don’t know the concrete lifetimes of the references
that will be passed in, so we can’
t look at the scopes as we did in Listings
10-17 and 10-18 to determine whether
the reference we return will always be valid. The
borrow checker can’t determine this either, because
it doesn’t know how the lifetimes of x
and y relate to the lifetime of the return value
. To fix this error, we’ll add
generic lifetime parameters that define the relationship between the references
so the borrow checker can perform its analysis.
