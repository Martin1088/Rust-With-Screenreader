// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;
enum Powerstate {
    Reboot,
    Off,
    Sleep,
    Shutdown,
    Hibernate,
}

fn input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect ("invalid");
    let result = buffer.trim().to_lowercase().to_owned();
    print_type_of(&result);
    return result;
}

fn check(input: &str) -> Result<Powerstate,String> {
    match input {
        "reboot" => Ok(Powerstate::Reboot),
        "off" => Ok(Powerstate::Off),
        "sleep" => Ok(Powerstate::Sleep),
        "shutdown" => Ok(Powerstate::Shutdown),
        "hibernate" => Ok(Powerstate::Hibernate),
        _ => Err("not valid".to_owned()),
    }
}

fn print_check(input: &str) {
    let state = check(&input);
    print_type_of(&state);
    match state {
        Ok(Powerstate::Off) => println!("is off"),
        Ok(Powerstate::Sleep) => println!("is a sleep"),
        Ok(Powerstate::Reboot) => println!("is rebooting"),
        Ok(Powerstate::Hibernate) => println!("is at rest"),
        Ok(Powerstate::Shutdown) => println!("is shutingdown"),
        Err(e) => println!("{e}"),


    }
}

fn print_type_of<T>(_:&T) {
    println!("Type: {:?}", std::any::type_name::<T>())
}

fn main() {
    let user = input();
    print_type_of(&user);
    print_check(&user);
}
