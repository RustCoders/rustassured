
fn add(x: i32, y: i32) -> i32 {
    // Equivalent to "return x + y;"
    x + y
}

fn main() {

    // The add function uses an expression to return a value, see above.
    println!("2 + 2 = {}", add(2,2));

    let addend = 2;
    // Expressions can be return a value from a conditional as well.
    let result = {
        if addend > 3 {
            addend
        } else {
            add(addend, addend)
        }
    };
    println!("The result was {}", result);

}