# Errors

Rust has 2 types of errors

1. Recoverable errors
1. Unrecoverable errors

Shortcuts for Panic and error

1. unwrap
1. expect

There is a difference between what the match expression from
Listing 9-6 does and what the ? operator
does: error values that have the ? operator called
on them go through the from function, defined in
the From trait in the standard library, which is
used to convert values from one type into another.
When the ? operator calls the from function, the
error type received is converted into the error type defined
in the return type of the current function. This
is useful when a function returns one error type to
represent all the ways a function might fail, even
if parts might fail for many different reasons.

The error message also mentioned that ? can be used
with Option<T> values as well. As
with using ? on Result, you can only use
? on Option in a function that returns an Option
. The behavior of the ? operator when called on
an Option<T> is similar to its behavior
when called on a Result<T, E>:
if the value is None, the None will be
returned early from the function at that point. If
the value is Some, the value inside the Some
is the resultant value of the expression, and the
function continues. Listing 9-11 has an example
of a function that finds the last character of the
first line in the given text.

The main function may return any types that implement the
std::process::Termination trait, which contains a
function report that returns an ExitCode. Consult the standard
library documentation for more information on implementing the Termination trait
for your own types.
