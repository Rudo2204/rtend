use clap::ArgMatches;
use rusqlite::{self, params, Connection};
use std::{process, str::FromStr};
use time;

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

        for entity in entity_iter {
            let tmp = entity.unwrap();
            println!(
                "Found id {}, created at {}",
                tmp.id,
                time::at(tmp.created).rfc3339()
            );
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

        for entity in entity_iter {
            let tmp = entity.unwrap();
            println!(
                "Found id {}, alias_list {}, alias_count {}, snippet_count {}, created at {}",
                tmp.id,
                tmp.alias_list,
                tmp.alias_count,
                tmp.snippet_count,
                time::at(tmp.created).rfc3339()
            );
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
            updated from relation where entity_id_a = (?1)
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

        for entity in entity_iter {
            let tmp = entity.unwrap();
            println!(
                "Found id {}, type {}, data {}, last_modified {}",
                tmp.id,
                tmp.data_type,
                tmp.data,
                time::at(tmp.last_modified).rfc3339()
            );
        }
    }

    Ok(())
}
