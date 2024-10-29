use rusqlite::{params, Connection, Result};

pub struct Hero {
    pub id: i32,
    pub name: String,
    pub power: i32,
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS hero (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            power INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_hero(conn: &Connection, hero: Hero) -> Result<()> {
    conn.execute(
        "INSERT INTO hero (id, name, power) VALUES (?1, ?2, ?3)",
        params![hero.id, hero.name, hero.power],
    )?;
    Ok(())
}

pub fn get_hero(conn: &Connection, hero_id: i32) -> Result<Hero> {
    let mut stmt = conn.prepare("SELECT id, name, power FROM hero WHERE id = ?1")?;
    let hero = stmt.query_row([hero_id], |row| {
        Ok(Hero {
            id: row.get(0)?,
            name: row.get(1)?,
            power: row.get(2)?,
        })
    })?;
    Ok(hero)
}

pub fn update_hero(conn: &Connection, hero_id: i32, new_power: i32) -> Result<()> {
    conn.execute(
        "UPDATE hero SET power = ?1 WHERE id = ?2",
        params![new_power, hero_id],
    )?;
    Ok(())
}

pub fn delete_hero(conn: &Connection, hero_id: i32) -> Result<()> {
    conn.execute("DELETE FROM hero WHERE id = ?1", params![hero_id])?;
    Ok(())
}
