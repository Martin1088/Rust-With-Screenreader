

use rusqlite;

pub fn table_create() -> rusqlite::Result<()>{
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

pub fn add_entry_database(name: &str, amount: &f64) {

}

pub fn show_entry_database() {

}

pub fn remove_entry_database() {

}

pub fn edit_entry_database() {

}


