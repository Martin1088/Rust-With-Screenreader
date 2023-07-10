fn main() {
    let number = vec![1, 2, 3, 4];
    println!("lenght: {}" ,number.len());
    let mut color = Vec::new();
    color.push("blue");
    color.push("green");
    color.push("black");
    color.push("red");
    println!("{:?}", color);
    println!("lenght: {}", color.len());
    for n in color {
        print!(" {}", n);
    }
    println!();

}
