Also note that the values we get from the calls
to `next` are immutable references to the values in the
vector. The `iter` method produces an iterator over immutable
references. If we want to create an iterator that
takes ownership of `v1` and returns owned values, we
can call `into_iter` instead of `iter`. Similarly, if
we want to iterate over mutable references, we can
call `iter_mut` instead of `iter`.
