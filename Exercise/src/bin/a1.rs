// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let firstname = "Martin";
    let lastname = "Jurk";
    println!("Yourname : {} {}" ,firstname, lastname);
    dbg!(firstname);
    first_name();
}
fn first_name() {
    println!("Martin");
}
