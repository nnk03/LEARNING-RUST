1. Panics: The scenarios in which the function being documented
   could panic. Callers of the function who don’
   t want their programs to panic should make sure they
   don’t call the function in these situations.

1. Errors: If the function returns a Result
   , describing the kinds of errors that might occur and
   what conditions might cause those errors to be returned can
   be helpful to callers so they can write code to
   handle the different kinds of errors in different ways.

1. Safety: If the function is unsafe to
   call (we discuss unsafety in Chapter 19), there
   should be a section explaining why the function is unsafe
   and covering the invariants that the function expects callers to
   uphold.
