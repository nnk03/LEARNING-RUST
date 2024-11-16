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
