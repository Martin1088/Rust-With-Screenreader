struct Temperature {
    degree_f: f64,
    degree_c: f64,
}
fn show_temp(temp: &Temperature) {
    println!("{} drgree F", temp.degree_f);
    println!("{} drgree C", temp.degree_c);
}
fn calc(temp: Temperature) {
    temp.degree_c = ((temp.degree_f - 32.0) * 5.0 ) /9.0;

}

fn main() {
    let mut hot = Temperature{
        degree_f: 9.99,
        degree_c: 0.00,
    };
    show_temp(&hot);
    calc(hot);
    show_temp(&hot);

}
