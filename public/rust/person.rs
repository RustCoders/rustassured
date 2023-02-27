pub struct Person {
    name: String,
    state: String,
}

impl Person {
    // See https://rust-unofficial.github.io/patterns/idioms/ctor.html
    pub fn new(name: String, state: String) -> Self {
        Self {
            name: name,
            state: state
        }
    }

    pub fn print(&self) {
        println!("{} lives in {}.", self.name, self.state);
    }
    
    // Note it's &mut, not mut&
    pub fn move_to(&mut self, state: String) {
        self.state = state;
    }
}

fn main() {
    let static_john = Person::new("John Lockwood".to_string(), "California".to_string());
    static_john.print();

    // Static John can't move! Next line is an error
    // static_john.move_to("North Carolina".to_string());

    // Mutable John can
    let mut john = Person::new("John Lockwood".to_string(), "California".to_string());

    john.print();

    john.move_to("North Carolina".to_string());
    john.print();
}