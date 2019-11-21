use clap::{value_t, ArgMatches};
use rusqlite;
use rusqlite::{params, Connection, NO_PARAMS};
use std::process;

use crate::utils;

pub fn add(args: &ArgMatches) {
    if args.is_present("add_entity") {
        add_new_entity(args.value_of("add_entity").unwrap());
    }
}

fn add_new_entity(name: &str) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

    conn.execute("INSERT INTO entity default values", NO_PARAMS)?;
    conn.execute("INSERT INTO alias (name) VALUES (?1)", params![name])?;
    println!("entity {} added", name);

    Ok(())
}
