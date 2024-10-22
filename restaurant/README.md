# Restaurant example in documentation

Items in a parent module can’t use the private items inside child
modules, but items in child modules can use the
items in their ancestor modules. This is because child
modules wrap and hide their implementation details, but the
child modules can see the context in which they’re defined.
(kind of like `protected` in C++????)

To continue with our metaphor, think
of the privacy rules as being like the back office
of a restaurant: what goes on in there is
private to restaurant customers, but office managers can see
and do everything in the restaurant they operate.

If we make a `struct` public, the members are still private

but in contrast, if we make an `enum` public, all of its variants are public
