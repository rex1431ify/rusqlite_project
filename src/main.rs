use rusqlite::{params, Connection, Result};

#[derive(Debug)]

struct People {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE People (
        id      INTEGER PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BLOB
        )",
        [],
    )?;

    let me = People {
        id: 0,
        name: "Rasmus".to_string(),
        data: None,
    };

    let _person1 = People {
        id: 1,
        name: "john doe".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO People (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    conn.execute(
        "INSERT INTO People (name, data) VALUES (?1, ?2)",
        params![_person1.name, _person1.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM People")?;
    let people_iter = stmt.query_map([], |row| {
        Ok(People {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for people in people_iter {
        println!("Found person {:?}", people.unwrap());
    }
    Ok(())
}