use rusqlite::Connection;
use sql_operations::{create_table, insert_hero, get_hero, update_hero, delete_hero, Hero};

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?; // Or use a file with `Connection::open("heroes.db")`

    // Create the table
    create_table(&conn)?;

    // Insert a new hero
    let hero = Hero {
        id: 1,
        name: String::from("Superman"),
        power: 100,
    };
    insert_hero(&conn, hero)?;

    // Read and display hero details
    let hero = get_hero(&conn, 1)?;
    println!("Hero ID: {}, Name: {}, Power: {}", hero.id, hero.name, hero.power);

    // Update the hero's power
    update_hero(&conn, 1, 120)?;

    // Confirm update
    let updated_hero = get_hero(&conn, 1)?;
    println!("Updated Hero Power: {}", updated_hero.power);

    // Delete the hero
    delete_hero(&conn, 1)?;

    Ok(())
}
