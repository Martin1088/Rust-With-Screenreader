use std::io;

pub fn input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("faild to read");
    return user_input.trim().to_owned();
}

pub fn menu() {
    println!("1) add a contact");
    println!("2) show all contacts");
    println!("3) remove a contact");
    println!("4) edit existing contacts");
    println!("5) quit");
    println!("type your selection");
}
pub fn show_contacts() {
    println!("contacts");
}


