// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id: i32,
    quantity: i32
}
fn display_id(grocery: &Grocery) {
    println!("The id is: {}", grocery.id);
}
fn display_quantity(grocery: &Grocery ) {
    println!("The qualitiy is : {}", grocery.quantity);
}
fn main() {
    let item = Grocery {
        id: 101,
        quantity: 20
    };
    display_id(&item);
    display_quantity(&item);
}
