Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:

1. Set up any needed data or state.
1. Run the code you want to test.
1. Assert that the results are what you expect.

You can’t use the #[should_panic] annotation
on tests that use Result<T, E>.
To assert that an operation returns an Err variant,
don’t use the question mark operator on the
Result<T, E> value. Instead,
use assert!(value.is_err())
.

## UNIT TESTS AND INTEGRATION TESTS
