use clap::ArgMatches;
use rusqlite::{self, params, Connection};
use std::{process, str::FromStr, unreachable};

use crate::utils;
pub fn delete(args: &ArgMatches) {
    if args.is_present("delete_entity") {
        let entity_id =
            u32::from_str(args.value_of("delete_entity").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an u32");
                process::exit(1);
            });

        match delete_entity(entity_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not delete entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("delete_alias") {
        let alias_id =
            u32::from_str(args.value_of("delete_alias").unwrap()).unwrap_or_else(|_err| {
                eprintln!("alias_id must be an u32");
                process::exit(1);
            });

        match delete_alias(alias_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not delete alias, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("delete_snippet") {
        let snippet_id =
            u32::from_str(args.value_of("delete_snippet").unwrap()).unwrap_or_else(|_err| {
                eprintln!("snippet_id must be an u32");
                process::exit(1);
            });

        match delete_snippet(snippet_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not delete snippet, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("delete_relation") {
        let relation_id =
            u32::from_str(args.value_of("delete_relation").unwrap()).unwrap_or_else(|_err| {
                eprintln!("relation_id must be an u32");
                process::exit(1);
            });

        match delete_relation(relation_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not delete relation, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("delete_relation_snippet") {
        let relation_snippet_id = u32::from_str(args.value_of("delete_relation_snippet").unwrap())
            .unwrap_or_else(|_err| {
                eprintln!("relation_snippet_id must be an u32");
                process::exit(1);
            });

        match delete_relation_snippet(relation_snippet_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not delete relation snippet, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn delete_entity(entity_id: u32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;
    let rows_returned = conn.execute("DELETE from entity where id = (?)", params![entity_id])?;

    match rows_returned {
        0 => println!(
            "entity id {} does not exist. Nothing got deleted!",
            entity_id
        ),
        1 => {
            println!("entity id `{}` deleted", entity_id);
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn delete_alias(alias_id: u32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;
    let rows_returned = conn.execute("DELETE from alias where id = (?)", params![alias_id])?;

    match rows_returned {
        0 => println!("alias id {} does not exist. Nothing got deleted!", alias_id),
        1 => {
            println!("alias id `{}` deleted", alias_id);
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn delete_snippet(snippet_id: u32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;
    let rows_returned = conn.execute("DELETE from snippet where id = (?)", params![snippet_id])?;

    match rows_returned {
        0 => println!(
            "snippet id {} does not exist. Nothing got deleted!",
            snippet_id
        ),
        1 => {
            println!("snippet id `{}` deleted", snippet_id);
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn delete_relation(relation_id: u32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;
    let rows_returned =
        conn.execute("DELETE from relation where id = (?)", params![relation_id])?;

    match rows_returned {
        0 => println!(
            "relation id {} does not exist. Nothing got deleted!",
            relation_id
        ),
        1 => {
            println!("relation id `{}` deleted", relation_id);
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn delete_relation_snippet(relation_snippet_id: u32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;
    let rows_returned = conn.execute(
        "DELETE from relation_snippet where id = (?)",
        params![relation_snippet_id],
    )?;

    match rows_returned {
        0 => println!(
            "relation snippet id {} does not exist. Nothing got deleted!",
            relation_snippet_id
        ),
        1 => {
            println!("relation snippet id `{}` deleted", relation_snippet_id);
        }
        _ => unreachable!(),
    }

    Ok(())
}
