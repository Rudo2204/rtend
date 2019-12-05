use clap::ArgMatches;
use rusqlite::{self, params, Connection};
use std::{io, process, str::FromStr};

use crate::item;
use crate::utils;

pub fn find(args: &ArgMatches) {
    if args.is_present("find_alias") {
        let name = args.value_of("find_alias").unwrap();
        match find_alias(name, args.is_present("verbose")) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not find entity, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn find_alias(name: &str, verbose: bool) -> rusqlite::Result<()> {
    if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand init");
        process::exit(1);
    }
    let conn = Connection::open(&utils::find_data_dir().unwrap().join("notes.db"))?;

    if !verbose {
        let mut stmt = conn.prepare(
            "SELECT id, name, entity_id, updated from alias where name like (?) order by name",
        )?;

        let alias_iter = stmt.query_map(params![name], |row| {
            Ok(item::EntityFound {
                id: row.get(0)?,
                name: row.get(1)?,
                entity_id: row.get(2)?,
                updated: row.get(3)?,
            })
        })?;

        let mut stdout = io::BufWriter::new(io::stdout());
        let row = "-".repeat(66);
        let mut header_printed = false;
        for alias in alias_iter {
            let tmp = alias.unwrap();
            if !header_printed {
                tmp.print_header(&mut stdout, &row).unwrap();
                header_printed = true;
            }
            tmp.print_content(&mut stdout).unwrap();
        }
    } else {
        let mut stmt = conn.prepare(
            "
            SELECT a.id, a.name, a.entity_id, (SELECT group_concat(b.name, '; ') from alias b
            where a.entity_id = b.entity_id and a.id != b.id) as other_alias, a.updated from alias a
            where a.name like (?) order by a.name
            ",
        )?;

        let alias_iter = stmt.query_map(params![name], |row| {
            Ok(item::EntityFoundLong {
                id: row.get(0)?,
                name: row.get(1)?,
                entity_id: row.get(2)?,
                other_alias: row.get(3)?,
                updated: row.get(4)?,
            })
        })?;

        let mut stdout = io::BufWriter::new(io::stdout());
        let row = "-".repeat(79);
        let mut header_printed = false;
        for alias in alias_iter {
            let tmp = alias.unwrap();
            if !header_printed {
                tmp.print_header(&mut stdout, &row).unwrap();
                header_printed = true;
            }
            tmp.print_content(&mut stdout).unwrap();
        }
    }

    Ok(())
}
