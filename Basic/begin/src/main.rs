fn main() {
    println!("Hello, world!");
    println!("Cargo ist bereit");
    let name = "Martin Jurk";
    let age = 2023 - 1988;
    println!("Ihr Name: {0}", name);
    println!("Sie sind {0} Jahre alt", age);
    let result = calc(2023, 1972);
    println!("Ergebnis : {}", result );
    print!("{}", counter(1));
}
fn calc(a: i32, b: i32) -> i32 {
    a - b
}
fn counter(number: i32) -> i32 {
    number + 1
}
