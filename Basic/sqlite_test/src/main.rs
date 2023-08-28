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

fn main() {

}
