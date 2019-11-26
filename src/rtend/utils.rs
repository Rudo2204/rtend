use directories::BaseDirs;
use rusqlite;
use rusqlite::{Connection, NO_PARAMS};
use std::{fs, path, process};

pub fn check_database_exists() -> bool {
    find_data_dir().unwrap().join("notes.db").exists()
}

pub fn check_first_time() -> bool {
    let rtend_data_dir = find_data_dir().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    !rtend_data_dir.exists()
}

pub fn find_data_dir() -> Result<path::PathBuf, &'static str> {
    if let Some(base_dir) = BaseDirs::new() {
        Ok(base_dir.data_dir().join("rtend"))
    } else {
        Err("Could not retrieve home directory. You maybe are using unsupported OS.")
    }
}

pub fn create_new_db(first_time: bool) -> rusqlite::Result<()> {
    let rtend_data_dir = find_data_dir().unwrap();

    println!(
        "rtend data does not exist, will now create one at: {}",
        rtend_data_dir.display()
    );

    if first_time {
        match fs::create_dir(&rtend_data_dir) {
            Ok(()) => (),
            Err(err) => eprintln!("Problem creating data directory: {}", err),
        }
    }

    let conn = Connection::open(&rtend_data_dir.join("notes.db"))?;

    // Importing schema
    conn.execute(
        "CREATE TABLE entity (
             id integer primary key autoincrement,
             created datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE snippet (
            id integer primary key,
            entity_id integer references entity(id),
            data text not null,
            created datetime not null default current_timestamp,
            updated datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE alias (
            id integer primary key,
            entity_id integer references entity(id),
            name varchar(255) not null,
            created datetime not null default current_timestamp,
            updated datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE relation (
            id integer primary key,
            entity_id_a integer not null references entity(id),
            entity_id_b integer not null references entity(id),
            created datetime not null default current_timestamp,
            updated datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE relation_snippet (
            id integer primary key,
            relation_id integer not null references entity(id),
            data text not null,
            created datetime not null default current_timestamp,
            updated datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    println!("Sucessully created database.");
    Ok(())
}
