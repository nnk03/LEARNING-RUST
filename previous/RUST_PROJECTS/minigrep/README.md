# MINIGREP

```
Note that std::env::args will panic if
any argument contains invalid Unicode. If your program needs
to accept arguments containing invalid Unicode, use std::
env::args_os instead. That function returns an iterator
that produces OsString values instead of String values. We
’ve chosen to use std::env::args
here for simplicity because OsString values differ per platform and
are more complex to work with than String values.

```

The organizational problem of allocating responsibility for multiple tasks to the
main function is common to many binary projects. As
a result, the Rust community has developed guidelines for
splitting the separate concerns of a binary program when main
starts getting large. This process has the following steps
:

1. Split your program into a main
   .rs file and a lib.rs file and
   move your program’s logic to lib.rs
   .
1. As long as your command line parsing
   logic is small, it can remain in main.
   rs.
1. When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the following:

1. Calling the command line parsing logic with the argument values
1. Setting up any other configuration
1. Calling a run function in lib.rs
1. Handling the error if run returns an error

This pattern is about separating concerns: main.rs
handles running the program and lib.rs handles all
the logic of the task at hand. Because you
can’t test the main function directly, this
structure lets you test all of your program’s
logic by moving it into functions in lib.rs
. The code that remains in main.rs will
be small enough to verify its correctness by reading it
. Let’s rework our program by following this
process.

In this section, we’ll add the searching
logic to the minigrep program using the test-driven
development (TDD) process with the following steps:

1. Write a test that fails and run it to
   make sure it fails for the reason you expect.
1. Write or modify just enough code to make the new
   test pass.
1. Refactor the code you just added or
   changed and make sure the tests continue to pass.
1. Repeat from step 1!
