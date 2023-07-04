struct Temperature {
    degree_f: f64,
    degree_c: f64,
}
fn show_temp(temp: &Temperature) {
    println!("{} drgree F", temp.degree_f);
    println!("{} drgree C", temp.degree_c);
}
fn calc(temp: f64) -> f64 {
    let result = ((temp - 32.0) * 5.0 ) /9.0;
    return result;

}

fn main() {
    let d = 99.9;
    let hot = Temperature{
        degree_f: d,
        degree_c: calc(d),
    };
    show_temp(&hot);
    show_temp(&hot);

}
