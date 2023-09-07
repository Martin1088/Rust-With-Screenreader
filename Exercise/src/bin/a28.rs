// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor{
    fn new(color: Color) -> Self { Self(color) }
}

#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor{
    fn new(color: Color) -> Self { Self(color) }
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor{
    fn new(color: Color) -> Self { Self(color) }
}

fn print_color_shirt(color: ShirtColor) { println!(" {:?}", color) }

fn print_color_shoes(color: ShoesColor) { println!(" {:?}", color) }

fn print_color_pants(color: PantsColor) { println!(" {:?}", color)}
 

fn main() {
    let shoes01 = ShoesColor::new(Color::Blue);
    let shirt01 = ShirtColor::new(Color::Green);
    let pants01 = PantsColor::new(Color::Black);


    print_color_pants(pants01);
    print_color_shirt(shirt01);
    print_color_shoes(shoes01);
}
