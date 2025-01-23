use rusqlite;

fn connect_database() {}

pub fn table_create() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("note.db")?;
    conn.execute(
        "create table bills(
        id integer primary key autoincrement,
        name TEXT,
        amount REAL
        )",
        (),
    )?;
    Ok(())
}

pub fn add_entry_database(values: (&String, f64)) -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("note.db")?;
    conn.execute(
        "insert into bills(name, amount)
        values (?, ?)",
        rusqlite::params![values.0, values.1],
    )?;
    Ok(())
}

pub fn show_entry_database() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("note.db")?;
    let result = conn.prepare("Select * from bills")?;

    let rows = result.query_map([], |row| row.get(0))?;
    Ok(())
}

pub fn remove_entry_database() {}

pub fn edit_entry_database() {}
