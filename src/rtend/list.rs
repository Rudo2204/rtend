use clap::ArgMatches;
use rusqlite::{self, params, Connection};
use std::{process, str::FromStr};

use crate::item;
use crate::item::ComfyTable;

pub fn list(args: &ArgMatches, conn: Connection) {
    if args.is_present("list_entity") {
        let entity_id =
            u32::from_str(args.value_of("list_entity").unwrap()).unwrap_or_else(|_err| {
                eprintln!("entity_id must be an u32");
                process::exit(1);
            });
        let verbosity_level = args.occurrences_of("verbose");

        match list_entity(conn, entity_id, verbosity_level) {
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

        match list_alias(conn, entity_id) {
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

        match list_snippet(conn, entity_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not list alias, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("list_relation") {
        let relation_id =
            u32::from_str(args.value_of("list_relation").unwrap()).unwrap_or_else(|_err| {
                eprintln!("relation_id must be an u32");
                process::exit(1);
            });

        match list_relation(conn, relation_id, args.is_present("verbose")) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not list relation, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("list_relation_snippet") {
        let relation_id = u32::from_str(args.value_of("list_relation_snippet").unwrap())
            .unwrap_or_else(|_err| {
                eprintln!("relation_id must be an u32");
                process::exit(1);
            });

        match list_relation_snippet(conn, relation_id) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not list relation snippet, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("verbose") {
        match list_verbose(conn) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not list verbosely, error: {}", e);
                process::exit(1);
            }
        }
    } else if args.is_present("list_stats") {
        match list_stats(conn) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Could not list stats, error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn list_verbose(conn: Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("
        SELECT id,
        (SELECT substr(group_concat(name, '; '), 0, 1000) from alias where entity_id = entity.id limit 4) as alias_list,
        (SELECT count(*) from alias where entity_id = entity.id) as alias_count,
        (SELECT count(*) from snippet where entity_id = entity.id) as snippet_count,
        created
        from entity order by 1
        ")?;

    let entity_iter = stmt.query_map(params![], |row| {
        Ok(item::EntityLong {
            id: row.get(0)?,
            alias_list: row.get(1)?,
            alias_count: row.get(2)?,
            snippet_count: row.get(3)?,
            created: row.get(4)?,
        })
    })?;

    let mut tmp_vec = Vec::new();
    for entity in entity_iter {
        tmp_vec.push(entity.unwrap());
    }
    let cmfs = item::ComfyStruct { data: tmp_vec };
    cmfs.print_comfy_table();

    Ok(())
}
fn list_entity(conn: Connection, entity_id: u32, verbosity_level: u64) -> rusqlite::Result<()> {
    // No verbosity level, basically just lists the created date
    if verbosity_level == 0 {
        let mut stmt = conn.prepare("SELECT * from entity where id = (?1)")?;
        let entity_iter = stmt.query_map(params![entity_id], |row| {
            Ok(item::Entity {
                id: row.get(0)?,
                created: row.get(1)?,
            })
        })?;

        let mut tmp_vec = Vec::new();
        for entity in entity_iter {
            tmp_vec.push(entity.unwrap());
        }
        let cmfs = item::ComfyStruct { data: tmp_vec };
        cmfs.print_comfy_table();

    // Equal to list entity long
    } else if verbosity_level == 1 {
        let mut stmt = conn.prepare("
        SELECT id,
        (SELECT substr(group_concat(name, '; '), 0, 1000) from alias where entity_id = entity.id limit 4) as alias_list,
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

        let mut tmp_vec = Vec::new();
        for entity in entity_iter {
            tmp_vec.push(entity.unwrap());
        }
        let cmfs = item::ComfyStruct { data: tmp_vec };
        cmfs.print_comfy_table();

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
            updated from relation where (entity_id_a = (?1) or entity_id_b = (?1))
            UNION ALL
            SELECT id, 'rs', data, updated from relation_snippet
            where relation_id in (SELECT id from relation where (entity_id_a = (?1) or entity_id_b = (?1)))
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

        let mut tmp_vec = Vec::new();
        for entity in entity_iter {
            tmp_vec.push(entity.unwrap());
        }
        let cmfs = item::ComfyStruct { data: tmp_vec };
        cmfs.print_comfy_table();
    }

    Ok(())
}

fn list_alias(conn: Connection, entity_id: u32) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, updated from alias where entity_id = (?)")?;

    let alias_iter = stmt.query_map(params![entity_id], |row| {
        Ok(item::Alias {
            id: row.get(0)?,
            name: row.get(1)?,
            updated: row.get(2)?,
        })
    })?;

    let mut tmp_vec = Vec::new();
    for alias in alias_iter {
        tmp_vec.push(alias.unwrap());
    }
    let cmfs = item::ComfyStruct { data: tmp_vec };
    cmfs.print_comfy_table();

    Ok(())
}

fn list_snippet(conn: Connection, entity_id: u32) -> rusqlite::Result<()> {
    let mut stmt =
        conn.prepare("SELECT id, data as snippet, updated from snippet where entity_id = (?)")?;

    let snippet_iter = stmt.query_map(params![entity_id], |row| {
        Ok(item::Snippet {
            id: row.get(0)?,
            data: row.get(1)?,
            updated: row.get(2)?,
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

fn list_relation(conn: Connection, relation_id: u32, verbose: bool) -> rusqlite::Result<()> {
    if !verbose {
        let mut stmt = conn.prepare(
            "SELECT id, entity_id_a, entity_id_b,
            updated from relation where id = (?)",
        )?;

        let relation_iter = stmt.query_map(params![relation_id], |row| {
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
            updated from relation where id = (?)",
        )?;

        let relation_iter = stmt.query_map(params![relation_id], |row| {
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

fn list_relation_snippet(conn: Connection, relation_id: u32) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, data as snippet, updated from relation_snippet where relation_id = (?)",
    )?;

    let relation_iter = stmt.query_map(params![relation_id], |row| {
        Ok(item::RelationSnippet {
            id: row.get(0)?,
            data: row.get(1)?,
            updated: row.get(2)?,
        })
    })?;

    let mut tmp_vec = Vec::new();
    for relation in relation_iter {
        tmp_vec.push(relation.unwrap());
    }
    let cmfs = item::ComfyStruct { data: tmp_vec };
    cmfs.print_comfy_table();

    Ok(())
}

fn list_stats(conn: Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT 'Entities', count(*) from entity
        UNION ALL
        SELECT 'Aliases', count(*) from alias
        UNION ALL
        SELECT 'Snippets', count(*) from snippet
        UNION ALL
        SELECT 'Relations', count(*) from relation
        UNION ALL
        SELECT 'Relation Snippets', count(*) from relation_snippet",
    )?;

    let stat_iter = stmt.query_map(params![], |row| {
        Ok(item::Stats {
            stat_type: row.get(0)?,
            count: row.get(1)?,
        })
    })?;

    let mut tmp_vec = Vec::new();
    for stat in stat_iter {
        tmp_vec.push(stat.unwrap());
    }
    let cmfs = item::ComfyStruct { data: tmp_vec };
    cmfs.print_comfy_table();

    Ok(())
}
