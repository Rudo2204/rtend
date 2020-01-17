use clap::ArgMatches;
use rusqlite::{self, params, Connection};
use std::{process, str::FromStr, unreachable};

use crate::utils;

pub fn delete(args: &ArgMatches, conn: Connection) {
    if args.is_present("delete_entity") {
        let entity_id =
            u32::from_str(args.value_of("delete_entity").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an u32");
                process::exit(1);
            });

        if args.is_present("force") {
            println!(
                "This operation will force delete everything related to this entity and cannot be undone"
            );
            if utils::get_yn_input().unwrap() {
                match force_delete_entity(conn, entity_id) {
                    Ok(()) => (),
                    Err(e) => {
                        eprintln!("Could not delete entity, error: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                println!("Aborted");
                process::exit(1);
            }
        } else {
            match delete_entity(conn, entity_id) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Could not delete entity, error: {}", e);
                    process::exit(1);
                }
            }
        }
    } else if args.is_present("delete_alias") {
        let alias_id =
            u32::from_str(args.value_of("delete_alias").unwrap()).unwrap_or_else(|_err| {
                eprintln!("alias_id must be an u32");
                process::exit(1);
            });

        match delete_alias(conn, alias_id) {
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

        match delete_snippet(conn, snippet_id) {
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

        if args.is_present("force") {
            println!(
            "This operation will force delete every relation snippets related to this relation_id and cannot be undone"
            );
            if utils::get_yn_input().unwrap() {
                match force_delete_relation(conn, relation_id) {
                    Ok(()) => (),
                    Err(e) => {
                        eprintln!("Could not delete relation, error: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                println!("Aborted");
                process::exit(1);
            }
        } else {
            match delete_relation(conn, relation_id) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Could not delete relation, error: {}", e);
                    process::exit(1);
                }
            }
        }
    } else if args.is_present("delete_relation_snippet") {
        let relation_snippet_id = u32::from_str(args.value_of("delete_relation_snippet").unwrap())
            .unwrap_or_else(|_err| {
                eprintln!("relation_snippet_id must be an u32");
                process::exit(1);
            });

        match delete_relation_snippet(conn, relation_snippet_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not delete relation snippet, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn delete_entity(conn: Connection, entity_id: u32) -> rusqlite::Result<()> {
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

fn delete_alias(conn: Connection, alias_id: u32) -> rusqlite::Result<()> {
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

fn delete_snippet(conn: Connection, snippet_id: u32) -> rusqlite::Result<()> {
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

fn delete_relation(conn: Connection, relation_id: u32) -> rusqlite::Result<()> {
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

fn delete_relation_snippet(conn: Connection, relation_snippet_id: u32) -> rusqlite::Result<()> {
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

fn force_delete_entity(conn: Connection, entity_id: u32) -> rusqlite::Result<()> {
    // Relation snippets
    let mut rows_returned = conn.execute(
        "DELETE from relation_snippet where relation_id in (SELECT id from relation where entity_id_a = (?1) or entity_id_b = (?1))",
        params![entity_id],
    )?;
    println!(
        "{} relation snippets of entity_id {} deleted",
        rows_returned, entity_id
    );

    // Relations
    rows_returned = conn.execute(
        "DELETE from relation where id in (SELECT id from relation where entity_id_a = (?1) or entity_id_b = (?1))",
        params![entity_id],
    )?;
    println!(
        "{} relations of entity_id {} deleted",
        rows_returned, entity_id
    );

    // Snippets
    rows_returned = conn.execute(
        "DELETE from snippet where id in (SELECT id from snippet where entity_id = (?))",
        params![entity_id],
    )?;
    println!(
        "{} snippets of entity_id {} deleted",
        rows_returned, entity_id
    );

    // Aliases
    rows_returned = conn.execute(
        "DELETE from alias where id in (SELECT id from alias where entity_id = (?))",
        params![entity_id],
    )?;
    println!(
        "{} aliases of entity_id {} deleted",
        rows_returned, entity_id
    );

    // Finally delete the entity itself
    delete_entity(conn, entity_id)?;

    Ok(())
}

fn force_delete_relation(conn: Connection, relation_id: u32) -> rusqlite::Result<()> {
    // Relation snippets
    let rows_returned = conn.execute(
        "DELETE from relation_snippet where relation_id = (?)",
        params![relation_id],
    )?;
    println!(
        "{} relation snippets of relation_id {} deleted",
        rows_returned, relation_id
    );

    // Then the relation itself
    delete_relation(conn, relation_id)?;

    Ok(())
}
