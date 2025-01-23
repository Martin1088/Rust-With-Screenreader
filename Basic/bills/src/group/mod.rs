mod help_fn;
mod model;

pub fn init_database() {
    model::table_create();
}

pub fn menu_prompt() {
    println!("1) add a bill");
    println!("2) show all bills");
    println!("3) remove abill");
    println!("4) edit existing bill");
    println!("5) quit");
    println!("type your selection");
}

fn add_bill() {
    println!("Type the name of your bill");
    let name: String = help_fn::input();
    println!("Type the amount with . :");
    let amount: f64;
    match help_fn::input_dec() {
        Some(dec) => amount = dec,
        None => return,
    }
    let insert_data = (&name, amount);
    let result = model::add_entry_database(insert_data);
    match result {
        Ok(o) => println!("success {:?}", o),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn print_data() {
    let result = model::show_entry_database();
    match result {
        Ok(rows) => {
            for entry in rows {
                println!("{:?}", entry);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

pub fn menu_main() {
    let mut choice: i32 = 0;
    menu_prompt();
    while choice != 5 {
        match help_fn::input_num() {
            Some(num) => choice = num,
            None => choice = 5,
        }
        match choice {
            1 => add_bill(),
            2 => print_data(),
            3 => model::remove_entry_database(),
            4 => model::edit_entry_database(),
            5 => choice = 5,
            _ => println!("Invalid Number"),
        };
    }
}
