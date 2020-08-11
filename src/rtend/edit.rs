use clap::ArgMatches;
use rusqlite::{self, params, Connection};
use std::{process, str::FromStr};

pub fn edit(args: &ArgMatches, conn: Connection) {
    if args.is_present("edit_alias") {
        let id = u32::from_str(args.value_of("edit_alias").unwrap()).unwrap_or_else(|_err| {
            eprintln!("alias_id must be an u32");
            process::exit(1);
        });
        match update_alias(conn, id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not update alias, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("edit_snippet") {
        let id = u32::from_str(args.value_of("edit_snippet").unwrap()).unwrap_or_else(|_err| {
            eprintln!("snippet_id must be an u32");
            process::exit(1);
        });
        match update_snippet(conn, id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not update snippet, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("edit_relation_snippet") {
        let id =
            u32::from_str(args.value_of("edit_relation_snippet").unwrap()).unwrap_or_else(|_err| {
                eprintln!("relation_snippet_id must be an u32");
                process::exit(1);
            });
        match update_relation_snippet(conn, id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not update relation snippet, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn update_alias(conn: Connection, id: u32) -> rusqlite::Result<()> {
    let old_data: String = conn.query_row_and_then(
        "SELECT name from alias where id = (?)",
        params![id],
        |data| data.get(0),
    )?;
    let edited_data = scrawl::with(&old_data).expect("Could not open editor");
    if edited_data.is_empty() {
        eprintln!("Edited data is empty. Aborted");
        process::exit(1);
    }
    println!("edited = {}", edited_data);

    let rows_returned = conn.execute(
        "UPDATE alias set name = (?1), updated = datetime('now') where id = (?2)",
        params![edited_data, id],
    )?;

    match rows_returned {
        1 => {
            println!("alias id `{}` updated", id);
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn update_snippet(conn: Connection, id: u32) -> rusqlite::Result<()> {
    let old_data: String = conn.query_row_and_then(
        "SELECT data from snippet where id = (?)",
        params![id],
        |data| data.get(0),
    )?;
    let edited_data = scrawl::with(&old_data).expect("Could not open editor");
    if edited_data.is_empty() {
        eprintln!("Edited data is empty. Aborted");
        process::exit(1);
    }

    let rows_returned = conn.execute(
        "UPDATE snippet set data = (?1), updated = datetime('now') where id = (?2)",
        params![edited_data, id],
    )?;

    match rows_returned {
        1 => {
            println!("snippet id `{}` updated", id);
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn update_relation_snippet(conn: Connection, id: u32) -> rusqlite::Result<()> {
    let old_data: String = conn.query_row_and_then(
        "SELECT data from relation_snippet where id = (?)",
        params![id],
        |data| data.get(0),
    )?;
    let edited_data = scrawl::with(&old_data).expect("Could not open editor");
    if edited_data.is_empty() {
        eprintln!("Edited data is empty. Aborted");
        process::exit(1);
    }

    let rows_returned = conn.execute(
        "UPDATE relation_snippet set data = (?1), updated = datetime('now') where id = (?2)",
        params![edited_data, id],
    )?;

    match rows_returned {
        1 => {
            println!("relation_snippet id `{}` updated", id);
        }
        _ => unreachable!(),
    }

    Ok(())
}
