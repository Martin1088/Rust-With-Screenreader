// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Red,
    Blue,
    Yellow
}
struct Dimension {
    width: f64,
    heigt: f64,
    length: f64
}
struct Shippingbox {
    color: Color,
    weight: f64,
    dimension: Dimension,
}
impl Shippingbox {
    fn show_box(&self) {
        println!("{} Color of the box ", self.color);
        println!("{} weight of the box", self.weight );
        }
    }
fn main() {
    let box_temp = Shippingbox {
    color: Color::Blue,
    weight: 100.0,
    dimension: Dimension { width: (10.0), heigt: (20.0), length: (40.0) }
    };
    box_temp.show_box();
}
