// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    name: String,
    color: String,
    age: i32,
}
fn print_name(word: &str) {
    println!(" {}", word);
}


fn main() {
    let test = vec![
        Person{
            name: "Tina".to_owned(),
            color: "red".to_owned(),
            age: 30,
        },
        Person{
            name: "Sam".to_owned(),
            color: "black".to_owned(),
            age: 5,
        },
        Person{
            name: "Tim".to_owned(),
            color: "blue".to_owned(),
            age: 10,
        },
        Person{
            name: "Emil".to_owned(),
            color: "green".to_owned(),
            age: 28,
        },
    ];
    for item in &test {
        if item.age <= 10 {
            print_name(&item.name);
            print_name(&item.color);
        }
    }
}
