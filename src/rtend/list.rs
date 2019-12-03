use clap::ArgMatches;
use rusqlite::{self, params, Connection};
use std::{io, process, str::FromStr};

use crate::item;
use crate::utils;

pub fn list(args: &ArgMatches) {
    if args.is_present("list_entity") {
        let entity_id =
            u32::from_str(args.value_of("list_entity").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an u32");
                process::exit(1);
            });
        let verbosity_level = args.occurrences_of("verbose");

        match list_entity(entity_id, verbosity_level) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not list entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("list_alias") {
        let entity_id =
            u32::from_str(args.value_of("list_alias").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an u32");
                process::exit(1);
            });

        match list_alias(entity_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not list alias, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("list_snippet") {
        let entity_id =
            u32::from_str(args.value_of("list_snippet").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an u32");
                process::exit(1);
            });

        match list_snippet(entity_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not list alias, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn list_entity(entity_id: u32, verbosity_level: u64) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }
    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

    // No verbosity level, basically just lists the created date
    if verbosity_level == 0 {
        let mut stmt = conn.prepare("SELECT * from entity where id = (?1)")?;
        let entity_iter = stmt.query_map(params![entity_id], |row| {
            Ok(item::Entity {
                id: row.get(0)?,
                created: row.get(1)?,
            })
        })?;

        let mut stdout = io::BufWriter::new(io::stdout());
        let row = "-".repeat(34);
        for entity in entity_iter {
            let tmp = entity.unwrap();
            tmp.print_header(&mut stdout, &row).unwrap();
            tmp.print_content(&mut stdout).unwrap();
        }

    // Equal to list entity long
    } else if verbosity_level == 1 {
        let mut stmt = conn.prepare("
        SELECT id,
        (SELECT substr(group_concat(name, '; '), 0, 40) from alias where entity_id = entity.id limit 4) as alias_list,
        (SELECT count(*) from alias where entity_id = entity.id) as alias_count,
        (SELECT count(*) from snippet where entity_id = entity.id) as snippet_count,
        created
        from entity where id = (?1) order by 1
        ")?;

        let entity_iter = stmt.query_map(params![entity_id], |row| {
            Ok(item::EntityLong {
                id: row.get(0)?,
                alias_list: row.get(1)?,
                alias_count: row.get(2)?,
                snippet_count: row.get(3)?,
                created: row.get(4)?,
            })
        })?;

        let mut stdout = io::BufWriter::new(io::stdout());
        let row = "-".repeat(80);
        let mut header_printed = false;
        for entity in entity_iter {
            let tmp = entity.unwrap();
            if !header_printed {
                tmp.print_header(&mut stdout, &row).unwrap();
                header_printed = true;
            }
            tmp.print_content(&mut stdout);
        }

    // Equal to list entity long long
    } else {
        let mut stmt = conn.prepare(
            "
            SELECT id, 'e' as type, cast(id as text) as data, created as last_modified from entity where id = (?1)
            UNION ALL
            SELECT id, 'a', name, updated from alias where entity_id = (?1)
            UNION ALL
            SELECT id, 's', data, updated from snippet where entity_id = (?1)
            UNION ALL
            SELECT id, 'r', (entity_id_a || ' | ' || entity_id_b) as 'a | b',
            updated from relation where entity_id_a = (?1) or entity_id_b = (?1)
            UNION ALL
            SELECT id, 'rs', data, updated from relation_snippet
            where id in (SELECT id from relation where entity_id_a = (?1) or entity_id_b = (?1))
            order by 2, 1
            ",
        )?;

        let entity_iter = stmt.query_map(params![entity_id], |row| {
            Ok(item::EntityLongLong {
                id: row.get(0)?,
                data_type: row.get(1)?,
                data: row.get(2)?,
                last_modified: row.get(3)?,
            })
        })?;

        let mut stdout = io::BufWriter::new(io::stdout());
        let row = "-".repeat(80);
        let mut header_printed = false;
        for entity in entity_iter {
            let tmp = entity.unwrap();
            if !header_printed {
                tmp.print_header(&mut stdout, &row).unwrap();
                header_printed = true;
            }
            tmp.print_content(&mut stdout);
        }
    }

    Ok(())
}

fn list_alias(entity_id: u32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }
    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

    let mut stmt = conn.prepare("SELECT id, name, updated from alias where entity_id = (?)")?;

    let alias_iter = stmt.query_map(params![entity_id], |row| {
        Ok(item::Alias {
            id: row.get(0)?,
            name: row.get(1)?,
            updated: row.get(2)?,
        })
    })?;

    let mut stdout = io::BufWriter::new(io::stdout());
    let row = "-".repeat(80);
    let mut header_printed = false;
    for alias in alias_iter {
        let tmp = alias.unwrap();
        if !header_printed {
            tmp.print_header(&mut stdout, &row).unwrap();
            header_printed = true;
        }
        tmp.print_content(&mut stdout).unwrap();
    }

    Ok(())
}

fn list_snippet(entity_id: u32) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }
    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

    let mut stmt =
        conn.prepare("SELECT id, data as snippet, updated from snippet where entity_id = (?)")?;

    let snippet_iter = stmt.query_map(params![entity_id], |row| {
        Ok(item::Snippet {
            id: row.get(0)?,
            data: row.get(1)?,
            updated: row.get(2)?,
        })
    })?;

    let mut stdout = io::BufWriter::new(io::stdout());
    let row = "-".repeat(80);
    let mut header_printed = false;
    for snippet in snippet_iter {
        let tmp = snippet.unwrap();
        if !header_printed {
            tmp.print_header(&mut stdout, &row).unwrap();
            header_printed = true;
        }
        tmp.print_content(&mut stdout);
    }

    Ok(())
}
