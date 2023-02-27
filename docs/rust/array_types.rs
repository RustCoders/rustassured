// Array type and initialization examples

fn main() {

    // Let compiler figure out that it's an
    // array of 4 i32 elements
    let implicit = [1, 2, 3, 4]; 
    show_four(implicit, "implicit");

    // Explicitly declare using type ; count syntax
    let explicit: [i32; 4] = [5, 6, 7, 8];
    show_four(explicit, "explicit");
    
    // Explicitly declare and initialize
    // an array with 5 elements, each set to the
    // number 33.
    let five_33s: [i8; 5] = [33; 5];    
    println!("five_33s: {:?}", five_33s);
}

// First parameter is array of exactly 4 values of type i32
fn show_four(array: [i32; 4], name: &str) {
    println!("{}: {:?}", name, array); 
}
