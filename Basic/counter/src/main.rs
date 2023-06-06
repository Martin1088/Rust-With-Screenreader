fn main() {
    let mut count = 0;
    loop {
        if count == 10 {
            break;
        }
        print!(" {} ", count);
        count = count + 1;
    }
    println!();
    // while example
    let mut reverse: i32 = 10;
    while reverse != 0 {
        print!(" {} ", reverse);
        reverse = reverse - 1;
    }
    println!();
}
