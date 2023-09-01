const MAX_SPEED: i32 = 9000;

fn driving_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED 
    } else {
        speed 
    }
}

fn main() {
    let d1 = 30000;
    println!("{:?}", driving_speed(d1));
}
