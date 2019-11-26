use atty::{is, Stream};
use clap::ArgMatches;
use rusqlite::{self, params, Connection, NO_PARAMS};
use std::io::{self, Read};
use std::{process, str::FromStr};

use crate::utils;

pub fn add(args: &ArgMatches) {
    if args.is_present("add_entity") {
        let name = args.value_of("add_entity").unwrap();
        match add_new_entity(name) {
            Ok(()) => println!("entity name `{}` added", name),
            Err(e) => {
                eprintln!("Could not add entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("add_alias") {
        let alias_args: Vec<_> = args.values_of("add_alias").unwrap().collect();
        let entity_id = i32::from_str(alias_args[0]).unwrap_or_else(|_err| {
            eprintln!("entity_id must be an i32");
            process::exit(1);
        });

        match add_alias_to_entity(entity_id, alias_args[1]) {
            Ok(()) => println!(
                "alias `{}` to entity_id `{}` added",
                alias_args[1], entity_id
            ),
            Err(e) => {
                eprintln!("Could not add alias to entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("add_relation") {
        let alias_args: Vec<_> = args.values_of("add_relation").unwrap().collect();

        let entity_id_a = i32::from_str(alias_args[0]).unwrap_or_else(|_err| {
            eprintln!("entity_id must be an i32");
            process::exit(1);
        });

        let entity_id_b = i32::from_str(alias_args[1]).unwrap_or_else(|_err| {
            eprintln!("entity_id must be an i32");
            process::exit(1);
        });

        match add_relation_two_entities(entity_id_a, entity_id_b) {
            Ok(()) => println!(
                "relation between entity_id `{}` and entity_id `{}` added",
                entity_id_a, entity_id_b
            ),
            Err(e) => {
                eprintln!("Could not add relation between two entities, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("add_snippet") {
        let entity_id =
            i32::from_str(args.value_of("add_snippet").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an i32");
                process::exit(1);
            });
        match add_new_snippet(entity_id) {
            Ok(()) => println!("new data snippet added to entity id `{}`", entity_id),
            Err(e) => {
                eprintln!("Could not add snippet to entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("add_relation_snippet") {
        let relation_id = i32::from_str(args.value_of("add_relation_snippet").unwrap())
            .unwrap_or_else(|_err| {
                eprintln!("relation_id must be an i32");
                process::exit(1);
            });
        match add_relation_snippet(relation_id) {
            Ok(()) => println!("new data snippet added to relation id `{}`", relation_id),
            Err(e) => {
                eprintln!("Could not add snippet to relation_snippet, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn add_alias_to_entity(entity_id: i32, name: &str) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

    conn.execute(
        "INSERT INTO alias (entity_id, name) VALUES
                 (?1, ?2)",
        params![entity_id, name],
    )?;

    Ok(())
}

fn add_new_entity(name: &str) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

    conn.execute("INSERT INTO entity default values", NO_PARAMS)?;
    conn.execute(
        "INSERT INTO alias (name, entity_id) VALUES
                 (?1, (SELECT seq from sqlite_sequence where name='entity'))",
        params![name],
    )?;

    Ok(())
}

fn add_relation_two_entities(id_a: i32, id_b: i32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

    conn.execute(
        "INSERT INTO relation (entity_id_a, entity_id_b) VALUES
                 (?1, ?2)",
        params![id_a, id_b],
    )?;

    Ok(())
}

fn add_new_snippet(entity_id: i32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    // Check if Stdin pipe is open, if it is then these messages will be omitted
    if is(Stream::Stdin) {
        if cfg!(taget_os = "windows") {
            println!("[Type in data for snippet - Termiate by Ctrl-Z and Return (Enter)]");
        } else {
            println!("[Type in data for snippet - Termiate by Return (Enter) and Ctrl-D]");
        }
    }

    let mut data = String::new();
    match io::stdin().read_to_string(&mut data) {
        Ok(_) => {
            let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

            conn.execute(
                "INSERT INTO snippet (data, entity_id) VALUES (?1, ?2)",
                params![data, entity_id],
            )?;
        }

        Err(err) => {
            eprintln!("Something went wrong reading input! Error: {}", err);
            process::exit(1);
        }
    }

    Ok(())
}

fn add_relation_snippet(relation_id: i32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    // Check if Stdin pipe is open, if it is then these messages will be omitted
    if is(Stream::Stdin) {
        if cfg!(taget_os = "windows") {
            println!("[Type in data for snippet - Termiate by Ctrl-Z and Return (Enter)]");
        } else {
            println!("[Type in data for snippet - Termiate by Return (Enter) and Ctrl-D]");
        }
    }

    let mut data = String::new();
    match io::stdin().read_to_string(&mut data) {
        Ok(_) => {
            let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

            conn.execute(
                "INSERT INTO relation_snippet (data, relation_id) VALUES (?1, ?2)",
                params![data, relation_id],
            )?;
        }

        Err(err) => {
            eprintln!("Something went wrong reading input! Error: {}", err);
            process::exit(1);
        }
    }

    Ok(())
}
