use demo::*;
mod range;

fn main() {
    // let range = 1..=3;
    range::test_range();
    println!();
    let merlin = Dog::new("Merlin", &12);
    merlin.hello();
    let john = Person::new("John", &60);
    john.hello();
    let d_p1 = Person::default();
    d_p1.hello();
    let d_d1 = Dog::default();
    d_d1.hello();
}
