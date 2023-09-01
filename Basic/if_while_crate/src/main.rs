use std::time::Duration;
use humantime::format_duration;


enum Color {
    Red,
    Blue,
    Yellow,
}

fn main() {
    let c1 = Color::Red;
    if let Color::Red = c1 { println!("Color Red"); }
    let v1 = vec![5, 6, 3, 3 , 33];
    let mut v1_iter = v1.iter();
    while let Some(num) = v1_iter.next() {
        print!(" {:?} ", num);
    }
    // humantime
    let d = Duration::from_secs(9876);
    println!("Time: {0}", format_duration(d));


}
