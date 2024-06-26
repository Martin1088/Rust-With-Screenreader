// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
struct Customer {
    name: String,
    age: i32,
}

fn check_age(input: &Customer) -> Result<String, String> {
    println!("Name: {:?}", input.name);
    match input.age {
        0..=21 => Err("You are to young".to_owned()),
        _ => Ok("You are old enough!".to_owned())

    }
}

fn main() {
    let test = Customer{
        name: "Merlin Jurk".to_owned(),
        age: 22
    };
    let result = check_age(&test);
    println!("{:?}", result);

}
