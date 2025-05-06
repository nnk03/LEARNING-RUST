# Fearless Concurrency

Here are the topics we’ll cover in this chapter:

1. How to create threads to run multiple pieces of code at the same time
1. Message-passing concurrency, where channels send messages between threads
1. Shared-state concurrency, where multiple threads have access to some piece of data
1. The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

Programming languages implement threads in a few different ways, and many operating systems provide an API the language can call for creating new threads. The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread. There are crates that implement other models of threading that make different tradeoffs to the 1:1 model.

One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data. Here’s the idea in a slogan from the Go language documentation: “Do not communicate by sharing memory; instead, share memory by communicating.”

# Why Rc<T> is not safe to share across threads

Unfortunately, Rc<T> is not safe to share across threads. When Rc<T> manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped. But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread. This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it. What we need is a type exactly like Rc<T> but one that makes changes to the reference count in a thread-safe way.

# Atomic Reference Counting with Arc<T>

Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use Mutex<T>. Recall in Chapter 15 that using Rc<T> came with the risk of creating reference cycles, where two Rc<T> values refer to each other, causing memory leaks. Similarly, Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever. If you’re interested in deadlocks, try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for mutexes in any language and have a go at implementing them in Rust. The standard library API documentation for Mutex<T> and MutexGuard offers useful information.

# Sync and Send Traits

The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads. In other words, any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread. Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.

The smart pointer Rc<T> is also not Sync for the same reasons that it’s not Send. The RefCell<T> type (which we talked about in Chapter 15) and the family of related Cell<T> types are not Sync. The implementation of borrow checking that RefCell<T> does at runtime is not thread-safe. The smart pointer Mutex<T> is Sync and can be used to share access with multiple threads as you saw in the “Sharing a Mutex<T> Between Multiple Threads” section.
