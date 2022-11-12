# Displaying Custom Types (Structs) in Rust

In [An Example Program Using Structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html), the Rust Book showed some of the error messages we could get when trying to print a struct to the console. 

That resource left unanswered the question of how to implement std::fmt::Display, so we added that in the version below.

```rust
{{#include rust/rect_example.rs}}
```
