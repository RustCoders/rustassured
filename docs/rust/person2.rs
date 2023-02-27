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
fn pass_ownership_non_mutable(person: Person) {
    person.print();

}

fn pass_ownership_mutably(mut person: Person) {
    person.move_to("New York".to_string());
    person.print();
}

// ANCHOR: pass_reference
fn pass_reference_non_mutable(person: &Person) {
    person.print();
}

fn pass_reference_mutably(person: &mut Person) {
    person.move_to("Washington".to_string());
    person.print();    
}
// ANCHOR_END: pass_reference

fn main() {
    // ANCHOR:  call_move
    // Pass a person not by reference
    let john2 = Person::new("John".to_string(), "California".to_string());
    pass_ownership_non_mutable(john2);
    // Can't do -- borrowed after move:  E0382:
    // john2.print();


    // Note we don't need to declare this as mut here
    // for that it matches the function signature of pass_ownership_mutually
    let john3 = Person::new("John ".to_string(), "California".to_string());
    pass_ownership_mutably(john3);
    // Can't do -- borrowed after move:  E0382:
    // john3.print();
    // ANCHOR_END:  call_move

    // ANCHOR:  call_reference
    let mut john3 = Person::new("John ".to_string(), "California".to_string());
    pass_reference_mutably(&mut john3);

    // Reusing, ownership not transfered!
    pass_reference_non_mutable(&john3);
    // ANCHOR_END:  call_reference

}