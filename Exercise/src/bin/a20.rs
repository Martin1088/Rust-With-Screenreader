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

impl Powerstate {
    fn new(state: &str) -> Option<Powerstate> {
        let state = state.trim().to_lowercase();
        print_type_of(&state);
        print_type_of(&state.as_str());
        match state.as_str() {
            "reboot" => Some(Powerstate::Reboot),
            "off" => Some(Powerstate::Off),
            "sleep" => Some(Powerstate::Sleep),
            "shutdown" => Some(Powerstate::Shutdown),
            "hibernate" => Some(Powerstate::Hibernate),
        _ => None,
        }
    }

}

fn input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.to_owned())
}

fn print_check(state: Powerstate) {
    print_type_of(&state);
    match state {
        Powerstate::Off => println!("is off"),
        Powerstate::Sleep => println!("is a sleep"),
        Powerstate::Reboot => println!("is rebooting"),
        Powerstate::Hibernate => println!("is at rest"),
        Powerstate::Shutdown => println!("is shutingdown"),
    }
}

fn print_type_of<T>(_:&T) {
    println!("Type: {:?}", std::any::type_name::<T>())
}


fn main() {
    println!("Type in a Powerstate");
    match input() {
        Ok(result) => {
            print_type_of(&result);
            match Powerstate::new(&result) {
                Some(state) => print_check(state),
                None => println!("invalid"),
            }
        },
        Err(e) => println!("{e}"),
    }
}
