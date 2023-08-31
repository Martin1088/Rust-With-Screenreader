use rusqlite;

fn table_create() -> rusqlite::Result<()>{
    let conn = rusqlite::Connection::open("note.db")?;
    conn.execute(
        "create table bills(
        id integer primary key autoincrement,
        name varchar(50),
        amount decimal(10,2)
        )",
        (),
        )?;
    Ok(())
}

fn input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("faild to read");
    return user_input.trim().to_owned();
}

fn menu() {
    println!("1) add a bill");
    println!("2) show all bills");
    println!("3) remove abill");
    println!("4) edit existing bill");
    println!("5) quit");
    println!("type your selection");
}

fn add_bill() -> Bill {
    println!("Type the name of your bill");
    let name: String = input();
    println!("Type the amount with . :");
    let amount: f64 = input().parse::<f64>().expect("not a valid number");
    return Bill::new(&name, &amount)
}


fn main() {
    table_create();
}
