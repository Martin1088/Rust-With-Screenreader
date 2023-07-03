// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
exit_program(true);
}
fn exit_program(choose: bool) {
    match choose {
    true => println!("Youe exited the programm"),
    false => println!(""),
    _ => println!("There is a problem"),
    }
}
