// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    show(7);
}
fn show(number: i32) {
    if number < 5 {
        println!("lower than 5");
    } else if number > 5 {
        println!("higher then 5");
    } else {
        println!("exactly 5");
    }
}
