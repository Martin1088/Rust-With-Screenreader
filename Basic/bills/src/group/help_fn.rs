use std::io;

pub fn input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("faild to read");
    return user_input.trim().to_owned();
}

pub fn input_num() -> Option<i32>  {
    match input().parse::<i32>() {
        Ok(num) => Some(num),
        Err(e) => None,
    }
}

pub fn input_dec() -> Option<f64>  {
    match input().parse::<f64>() {
        Ok(num) => Some(num),
        Err(e) => None,
    }
}




