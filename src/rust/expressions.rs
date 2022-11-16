
fn add(x: i32, y: i32) -> i32 {
    // Equivalent to "return x + y;"
    x + y
}

fn main() {
    println!("Hello!");
    println!("2 + 2 = {}", add(2,2));
    let addend = 4;

    // Expressions can be returned from a conditional
    let result = {
        if addend > 3 {
            addend
        } else {
            add(addend, addend)
        }
    };
    println!("The result was {}", result);

}