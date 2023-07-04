struct Temperature {
    degree_f: f64,
    degree_c: f64,
}
impl Temperature{
    fn show_temp(temp: &Temperature) {
        print!("{} degree F", temp.degree_f);
        print!("{} degree C", temp.degree_c);
    }
}
fn main() {

}
