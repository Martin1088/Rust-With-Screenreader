trait Noise {
    fn hello(&self);
}

struct Person {
    name: String,
    age: i32,
}

impl Person{
    fn new(name: &str, age: &i32) -> Person {
        Person {name: name.to_string(), age: age.clone()}
    }
} 

impl Noise for Person {
    fn hello(&self) { println!("Good morning!") }
}

impl Default for Person { 
    fn default() -> Self { Self{name: "May Mustermann".to_owned(), age: 100  } }
}

struct Dog {
    name: String,
    age: i32,
}

impl Dog {
    fn new(name: &str, age: &i32) -> Self {
        Self {name: name.to_string(), age: age.clone() }
    }
}

impl Noise for Dog { 
    fn hello(&self) { println!("woof Woof!") } 
}

impl Default for Dog {
    fn default() -> Self { Self{name: "cheaky".to_owned(), age: 1 } }
}

fn main() {
    let range = 1..=3;
    for num in 1..=6 {
        print!(" {:?} " , num);
    }

    for letter in 'a'..'q' {
        print!(" {:?} ", letter );
    }
    let merlin = Dog::new("Merlin", &12);
    merlin.hello();
    let john = Person::new("John", &60);
    john.hello();
    let d_p1 = Person::default();
    d_p1.hello();
    let d_d1 = Dog::default();
    d_d1.hello();
}
