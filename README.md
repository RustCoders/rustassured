# RustAssured

This is the code for the [RustAssured](https://rustassured.com) site.
It's currently written in [mdbook](https://rust-lang.github.io/mdBook/), a tool in Rust for writing books which forms the basis of the [Rust Book](https://github.com/rust-lang/book). 

## Getting started:
* Install mdbook as documented here under [Build from source using Rust](https://rustc-lang.github.io/mdBook/guide/installation.html#build-from-source-using-rust).
* To view the book:
```
mdbook serve --open
```

## CONTRIBUTIONS WELCOME

It's early days, so we are interested in almost anything about Rust, even rough "I hacked on this thing" notes, to get the Interwebs familiar with our (eventually-cool) site.

Writing an article basically involves:

* Add a markdown file in the src directory.  Code blocks marked as follows will automatically enable running code interactively using Rust Playground:

<pre>
```rust
// Your code here
```
</pre>

```mdbook serve --open``` as described above will run it on http://localhost:3000, with live update.  Easy peasy.

To include a link in the sidebar, you can add it to src/SUMMARY.md.

(You can also create a repo in this org if you need to illustrate something that can't be captured easily in mdbook.  If you need an organization invite, [enter an issue here](https://github.com/RustCoders/rustassured/issues)).

To do: John needs to write a CI/CD pipeline for main branch. Meantime he has a shell script so will do it manually when reviewing the PR.

