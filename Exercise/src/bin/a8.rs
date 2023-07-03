// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavour {
    Sweet,
    Fruity,
    Soure,
}
struct Drinks {
    name: String,
    flavour : Flavour,
    onces: i32,
}
fn show(cocktail: Drinks) {
    println!("The name: {}", cocktail.name);
   match cocktail.flavour {
        Flavour::Sweet => println!(" Sweet "),
        Flavour::Fruity => println!("Fruity,"),
        Flavour::Soure => println!(" Soure,"),
   }
    println!("ones {}", cocktail.onces);
}
fn main() {
 let cocktail = Drinks{
        name : "Honeymoon".to_string(),
        flavour: Flavour::Sweet,
        onces: 10,
    };
 show(cocktail);

}
