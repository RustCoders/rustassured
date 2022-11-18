# Fun Rust Facts

This is just a collection of various notes.

Both Scott and John noticed how cool the ```rustc --explain <error>``` feature is.
## Array Type Declarations

Array type declarations and initializers are a bit different in Rust than in other languages.  Here are some examples

```rust
{{#include rust/array_types.rs}}
``` 

## Expressions

Expressions do not end in a semi-colon, and they return a value.  This can be used for function return values as well as interesting things like setting a value in a conditional expression.

```rust
{{#include ../examples/expressions.rs}}
``` 
