# RustAssured

This is the code for the [RustAssured](https://rustassured.com) site.
It's currently written in [mdbook](https://rust-lang.github.io/mdBook/), a tool in Rust for writing books which forms the basis of the [Rust Book](https://github.com/rust-lang/book). 

## Getting started:
* Install mdbook as documented here under [Build from source using Rust](https://rust-lang.github.io/mdBook/guide/installation.html#build-from-source-using-rust).
* To view the book:
```
mdbook serve --open
```

## CONTRIBUTIONS WELCOME

Writing an article basically involves:

* Add a markdown file in the src directory.  Code blocks marked as follows will automatically enable running code interactively using Rust Playground:

```rust
// Your code here
```

```mdbook serve --open``` as described above will run it on http://localhost:3000, with live update.  Easy peasy.

To include a link in the sidebar, you can add it to src/SUMMARY.md.

To do: John needs to write a CI/CD pipeline for main branch. Meantime he has a shell script so will do it manually when reviewing the PR.
