struct Temperature {
    degree_f: f64,
    degree_c: f64,
}
impl Temperature{
    fn freezing() -> Self {
        Self {
            degree_f: 32.0,
            degree_c: 0.0,
        }
    }
    fn show_temp(&self) {
        println!("{} degree F", self.degree_f);
        println!("{} degree C", self.degree_c);
    }
}
fn main() {
    let hot = Temperature{
        degree_f : 99.9,
        degree_c : 37.2,
           };
    hot.show_temp();
    let cold = Temperature::freezing();
    cold.show_temp();

}
