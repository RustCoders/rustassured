
## Passing Objects

One of the things that's "hard" about Rust when you first start learning it are the different ways you can pass an object to a function or return one, and the types associated with them.

For example, Strings come in a few different flavors, and you need to know which one to use to return them.:

```rust

// This works:
fn return_string() -> String {
    let not_global = "I return ok, because I'm not global.".to_string();
    not_global  // Funky Rust return syntax.
}

// This doesn't, uncomment to see the error
/*
fn return_broken() -> String {
    let broken = "Not OK -- I'm a global literal!";
    broken
}
*/

println!("{}", return_string());
```

## Mutable methods vs immutable methods
Before learning about passing objects, we need to create a basic "object" and test it, which has mutable and immutable methods.

{{#playground rust/person.rs}}


## Passing Objects
Basically there are about four ways things can get passed around:
* Directly non-mutable
* Directly mutable
* By reference non-mutable
* By reference mutable

### Passing objects directly 
This MOVES the object to a new owner.  If you do this, you can't use
the reference any more.  I.e.  Can do it mutably or not.

```
fn pass_ownership_non_mutable(person: Person) {
    person.print();
}

fn pass_ownership_mutably(mut person: Person) {
    person.move_to("New York".to_string());
}
```

Calling:

```    
{{#include rust/person2.rs:call_move}}
``` 

### Passing by reference

```    
{{#include rust/person2.rs:pass_reference}}
``` 

Calling:

```    
{{#include rust/person2.rs:call_reference}}
``` 