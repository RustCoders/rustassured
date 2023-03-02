// A copied example from Rust docs
// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::result::Result;
//use std::error::Error;

fn get_name() -> Result<&'static str, &'static str> {
    Ok("John")
}


fn main() {

    // Create a path to the desired file
    // Debugging in VS Code, default starting path is workspace root.
    let path = Path::new("Cargo.toml");
    let display = path.display();

    let name = get_name();
    println!("Hello, {}", name.unwrap());

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    

    
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
