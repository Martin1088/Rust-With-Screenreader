extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    // random number
    let rng = rand::thread_rng;
    let secret: i32 = rng().gen_range(1..100);
    let mut counter = 10; // how many time you can guess
    let user_input: i32;
    println!("Guess and Type a number between 0 and 100");
    while counter != 0 {
       let user: i32 = input_from_user();
       if user == secret {
           println!("This number {} is correct!!!", secret);
           break;
       } else if user > secret {
           println!("The number is to high");
       } else if user < secret {
           println!("Number to low");
       } else {
           println!("Error");
       }

        counter = counter - 1;
    }
}
//
fn input_from_user() -> i32{
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read");
    let user_number: i32 = user_input.trim().parse::<i32>().expect("invalid input");
    return user_number;

}
