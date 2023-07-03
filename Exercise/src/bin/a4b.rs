// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    decision(4);
}
fn decision(number: i32) {
    match number {
        1 => println!("Azubis"),
        2 => println!("worker"),
        _ => println!("all others"),
    }
}
