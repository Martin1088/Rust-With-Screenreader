struct Temperature {
    degree_f: f64,
    degree_c: f64,
}
impl Temperature{
    fn show_temp(&self) {
        print!("{} degree F", self.degree_f);
        print!("{} degree C", self.degree_c);
    }
}
fn main() {
    let hot = Temperature{
        degree_f : 99.9,
        degree_c : 0.00,
    };
    hot.show_temp();
}
