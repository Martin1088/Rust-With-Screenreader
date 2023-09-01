mod group;

pub trait Noise {
    fn hello(&self);
}

pub struct Person {
    name: String,
    age: i32,
}

impl Person{
    pub fn new(name: &str, age: &i32) -> Person {
        Person {name: name.to_string(), age: age.clone()}
    }
} 

impl Noise for Person {
    fn hello(&self) { 
        println!("Good morning!");
        group::p1::test_next();
    }
}

impl Default for Person { 
    fn default() -> Self { Self{name: "May Mustermann".to_owned(), age: 100  } }
}

pub struct Dog {
    name: String,
    age: i32,
}

impl Dog {
    pub fn new(name: &str, age: &i32) -> Self {
        Self {name: name.to_string(), age: age.clone() }
    }
}

impl Noise for Dog { 
    fn hello(&self) { 
        println!("woof Woof!");
        group::p1::test_next();
    } 
}

impl Default for Dog {
    fn default() -> Self { Self{name: "cheaky".to_owned(), age: 1 } }
}

