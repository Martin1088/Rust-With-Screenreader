extern crate rand;
use rand::Rng;
use std::io;

fn main() {
let mut number = rand::thread_rng();
println!("The number is {}", number.gen_range(1..40));
let mut user_input = String::new();
io::stdin()
    .read_line(&mut user_input)
    .expect("failed to read");
let user_number = user_input.trim();
match user_number.parse::<u32>() {
    Ok(i) => println!("The number is {} from you", i),
    Err(..) => println!("This is no integer {} ", user_number),
};
}
