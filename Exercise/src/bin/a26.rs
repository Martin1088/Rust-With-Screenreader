// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats
use chrono;

fn main() {
    let test_time = chrono::offset::Local::now();
    let test_date = chrono::offset::Local::today();
    dbg!(test_time);
    dbg!(test_date);
    println!("{0}", test_time);
    println!("{0}", test_date);
    // format
    println!("{0}", test_time.format("%a %b %e %T %Y").to_string());
    println!("{0}", test_date.format("%a %b %e %Y").to_string() );

}
