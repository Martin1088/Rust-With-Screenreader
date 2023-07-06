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
impl Color {
    fn print(&self) {
        match self {
            Color::Blue => println!("blue"),
            Color::Red => println!("red"),
            Color::Yellow => println!("yellow")

        }
    }
}
struct Dimension {
    width: f64,
    height: f64,
    length: f64
}
impl Dimension{
    fn print(&self) {
        println!("widht: {}", self.width);
        println!("length: {}", self.length);
        println!("height: {}", self.height);
    }
}
struct Shippingbox {
    color: Color,
    weight: f64,
    dimension: Dimension,
}
impl Shippingbox {
    fn new(color: Color, weight: f64, dimension: Dimension) -> Self {
            Self {
                color,
                weight,
                dimension
            }
        }
    fn print(&self) {
        self.color.print();
        self.dimension.print();
        println!("weight: {}", self.weight);
    }
    }
fn main() {
    let small_box = Dimension {
    width: 20.0,
    length: 40.0,
    height: 10.0
    };
    let my_box = Shippingbox::new(Color::Yellow, 20.0, small_box);
    my_box.print();

}
