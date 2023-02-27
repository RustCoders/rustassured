fn main() {
    let mut i: isize = 0;
    let result = loop {
        i = i + 1;
        println!("Again!");
        if i > 9 {
            break "Variable 'i' got to 10, quitting!";
        }
    };
    println!("{}", result);
}