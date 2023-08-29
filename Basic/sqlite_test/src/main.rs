use rusqlite;

fn table_create() -> rusqlite::Result<()>{
    let conn = rusqlite::Connection::open("dogs.db")?;
    conn.execute(
        "create table dogs(
        id integer primary key,
        name varchar(50),
        place varchar(50)
            )",
           (),
            )?;
    Ok(())
}

fn print_type_of<T>(_:&T) {
    println!("Type: {:?}", std::any::type_name::<T>())
}


fn main() {
    let res = table_create();
    print_type_of(&res);
}
