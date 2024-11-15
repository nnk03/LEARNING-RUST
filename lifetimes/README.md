We want the signature to express the following constraint:
the returned reference will be valid as long as both
the parameters are valid. This is the relationship between
lifetimes of the parameters and the return value. We
’ll name the lifetime 'a and then add
it to each reference,

We covered a lot in this chapter! Now that
you know about generic type parameters, traits and trait
bounds, and generic lifetime parameters, you’re
ready to write code without repetition that works in many
different situations. Generic type parameters let you apply the
code to different types. Traits and trait bounds ensure
that even though the types are generic, they’
ll have the behavior the code needs. You learned
how to use lifetime annotations to ensure that this flexible
code won’t have any dangling references. And
all of this analysis happens at compile time, which
doesn’t affect runtime performance!
