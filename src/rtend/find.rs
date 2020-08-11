use clap::ArgMatches;
use rusqlite::{self, params, Connection};
use std::{process, str::FromStr};

use crate::item;
use crate::item::ComfyTable;

pub fn find(args: &ArgMatches, conn: Connection) {
    if args.is_present("find_alias") {
        let name = args.value_of("find_alias").unwrap();
        match find_alias(conn, name, args.is_present("verbose")) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not find entity, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("find_relation") {
        let entity_id =
            u32::from_str(args.value_of("find_relation").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an u32");
                process::exit(1);
            });

        match find_relation(conn, entity_id, args.is_present("verbose")) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not find relation, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("find_snippet") {
        let snippet_string = args.value_of("find_snippet").unwrap();
        match find_snippet(conn, snippet_string) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not find relation, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("find_relation_snippet") {
        let snippet_string = args.value_of("find_relation_snippet").unwrap();
        match find_relation_snippet(conn, snippet_string) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not find relation, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn find_alias(conn: Connection, name: &str, verbose: bool) -> rusqlite::Result<()> {
    if !verbose {
        let mut stmt = conn.prepare(
            "SELECT id, name, entity_id, updated from alias where name like '%' || ? || '%' order by name",
        )?;

        let entity_iter = stmt.query_map(params![name], |row| {
            Ok(item::EntityFound {
                id: row.get(0)?,
                name: row.get(1)?,
                entity_id: row.get(2)?,
                updated: row.get(3)?,
            })
        })?;

        let mut tmp_vec = Vec::new();
        for entity in entity_iter {
            tmp_vec.push(entity.unwrap());
        }
        let cmfs = item::ComfyStruct { data: tmp_vec };
        cmfs.print_comfy_table();
    } else {
        let mut stmt = conn.prepare(
            "SELECT a.id, a.name, a.entity_id, (SELECT group_concat(b.name, '; ') from alias b
            where a.entity_id = b.entity_id and a.id != b.id) as other_alias, a.updated from alias a
            where a.name like '%' || ? || '%' order by a.name",
        )?;

        let entity_iter = stmt.query_map(params![name], |row| {
            Ok(item::EntityFoundLong {
                id: row.get(0)?,
                name: row.get(1)?,
                entity_id: row.get(2)?,
                other_alias: row.get(3).unwrap_or_else(|_| "".to_string()),
                updated: row.get(4)?,
            })
        })?;

        let mut tmp_vec = Vec::new();
        for entity in entity_iter {
            tmp_vec.push(entity.unwrap());
        }
        let cmfs = item::ComfyStruct { data: tmp_vec };
        cmfs.print_comfy_table();
    }

    Ok(())
}

fn find_relation(conn: Connection, entity_id: u32, verbose: bool) -> rusqlite::Result<()> {
    if !verbose {
        let mut stmt = conn.prepare(
            "SELECT id, entity_id_a, entity_id_b,
            updated from relation where (entity_id_a = (?1) or entity_id_b = (?1))",
        )?;

        let relation_iter = stmt.query_map(params![entity_id], |row| {
            Ok(item::Relation {
                id: row.get(0)?,
                entity_id_a: row.get(1)?,
                entity_id_b: row.get(2)?,
                updated: row.get(3)?,
            })
        })?;

        let mut tmp_vec = Vec::new();
        for relation in relation_iter {
            tmp_vec.push(relation.unwrap());
        }
        let cmfs = item::ComfyStruct { data: tmp_vec };
        cmfs.print_comfy_table();
    } else {
        let mut stmt = conn.prepare(
            "SELECT id,
            entity_id_a, (SELECT group_concat(name, '; ') from alias where entity_id = entity_id_a limit 4) as alias_list_a,
            entity_id_b, (SELECT group_concat(name, '; ') from alias where entity_id = entity_id_b limit 4) as alias_list_b,
            updated from relation where (entity_id_a = (?1) or entity_id_b = (?1))",
        )?;

        let relation_iter = stmt.query_map(params![entity_id], |row| {
            Ok(item::RelationLong {
                id: row.get(0)?,
                entity_id_a: row.get(1)?,
                alias_list_a: row.get(2)?,
                entity_id_b: row.get(3)?,
                alias_list_b: row.get(4)?,
                updated: row.get(5)?,
            })
        })?;

        let mut tmp_vec = Vec::new();
        for relation in relation_iter {
            tmp_vec.push(relation.unwrap());
        }
        let cmfs = item::ComfyStruct { data: tmp_vec };
        cmfs.print_comfy_table();
    }

    Ok(())
}

fn find_snippet(conn: Connection, string: &str) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, data as snippet, entity_id, updated from snippet where data like '%' || ? || '%'",
    )?;

    let snippet_iter = stmt.query_map(params![string], |row| {
        Ok(item::SnippetFound {
            id: row.get(0)?,
            data: row.get(1)?,
            entity_id: row.get(2)?,
            updated: row.get(3)?,
        })
    })?;

    let mut tmp_vec = Vec::new();
    for snippet in snippet_iter {
        tmp_vec.push(snippet.unwrap());
    }
    let cmfs = item::ComfyStruct { data: tmp_vec };
    cmfs.print_comfy_table();

    Ok(())
}

fn find_relation_snippet(conn: Connection, string: &str) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, data as snippet, relation_id, updated from relation_snippet where data like '%' || ? || '%'",
    )?;

    let snippet_iter = stmt.query_map(params![string], |row| {
        Ok(item::RelationSnippetFound {
            id: row.get(0)?,
            data: row.get(1)?,
            relation_id: row.get(2)?,
            updated: row.get(3)?,
        })
    })?;

    let mut tmp_vec = Vec::new();
    for snippet in snippet_iter {
        tmp_vec.push(snippet.unwrap());
    }
    let cmfs = item::ComfyStruct { data: tmp_vec };
    cmfs.print_comfy_table();

    Ok(())
}
