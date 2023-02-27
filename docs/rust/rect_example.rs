use std::fmt;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle{{ width: {}, height: {} }}", self.width, self.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // For this to work, we needed fmt::Debug, given by #[derive(Debug)]
    println!("Using pretty_print:  rect1 is {:?}", rect1);
    
    // This too.  Here, make sure to use &rect1, or dbg! will take ownership!
    // Note that the output of this line will only appear 
    // if you use rustc to compile rect_example.rs and run it.
    // It won't show up in mdbook!
    dbg!(&rect1);

    // For this we needed fmt::Display, manually implemented.
    println!("Using display format: rect1 is {}", rect1);
}