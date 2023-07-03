use std::io;
fn dec2bin(n: i32) -> String {
    let mut dec:i32 = n;
    let mut result = String::new();
    while dec != 0 {
        let e = (dec % 2);
        result.push_str(&e.to_string()); 
        dec = dec /2;
    }
    return result;
}
fn main() {
// user Input
let mut user_input = String::new();
io::stdin()
    .read_line(& mut user_input)   
    .expect("failed to read");
let user_number: i32 = user_input.trim().parse::<i32>().expect("failed to read");
println!("The decimal numer ist : {} ", dec2bin(user_number));

   }
