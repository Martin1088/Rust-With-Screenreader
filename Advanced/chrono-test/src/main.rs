mod group;
use group::*;

fn main() {
    println!("start of day unix: {}", TimeFunctions::start_of_day());
    println!("End of day unix: {}", TimeFunctions::end_of_day());
    TimeFunctions::timestamp_rfc3339();
}
