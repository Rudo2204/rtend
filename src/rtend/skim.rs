#![cfg(target_family = "unix")]
use crate::item;
use regex::Regex;
use rusqlite::{self, params, Connection};
use skim::{Skim, SkimOptionsBuilder};
use std::io::{self, Cursor, Read, Seek, SeekFrom};
use std::str::FromStr;
use tempfile;

pub fn skim(conn: Connection) {
    let options = SkimOptionsBuilder::default()
        .preview_window(Some("down:50%"))
        .header_lines(2)
        .tabstop(Some("4"))
        .build()
        .unwrap();

    let re = Regex::new(r"^\s?(\d{1,5})").unwrap();

    let mut stmt = conn.prepare("
        SELECT id,
        (SELECT substr(group_concat(name, '; '), 0, 40) from alias where entity_id = entity.id limit 4) as alias_list,
        (SELECT count(*) from alias where entity_id = entity.id) as alias_count,
        (SELECT count(*) from snippet where entity_id = entity.id) as snippet_count,
        created
        from entity order by 1
        ").expect("Something went wrong when querying the database");

    let entity_iter = stmt
        .query_map(params![], |row| {
            Ok(item::EntityLong {
                id: row.get(0).unwrap(),
                alias_list: row.get(1).unwrap(),
                alias_count: row.get(2).unwrap(),
                snippet_count: row.get(3).unwrap(),
                created: row.get(4).unwrap(),
            })
        })
        .expect("Something went wrong when querying the database");

    let mut out_file = tempfile::tempfile().expect("Could not create temporary file");
    let row = "-".repeat(80);
    let mut header_printed = false;
    for entity in entity_iter {
        let tmp = entity.unwrap();
        if !header_printed {
            tmp.print_header(&mut out_file, &row)
                .expect("Could not write to temporary file");
            header_printed = true;
        }
        tmp.print_content(&mut out_file);
    }

    let mut buf = String::new();
    out_file
        .seek(SeekFrom::Start(0))
        .expect("Something went wrong seeking the temporary file");
    out_file
        .read_to_string(&mut buf)
        .expect("Something went wrong reading the temporary file");

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(buf))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    // TODO: This is the duplicated code way to do this, can probably clone and then call
    // list_verbose from list module but it will run into lifetime issue and connection sharing
    // See: https://github.com/jgallagher/rusqlite/issues/393
    let item = selected_items.iter().nth(0).unwrap();
    let full_output = item.get_output_text();
    let cap = re.captures(&full_output).unwrap().get(1).unwrap().as_str();
    let entity_id = u32::from_str(cap).unwrap();
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
    ).expect("Something went wrong when querying the database");

    let entity_iter = stmt
        .query_map(params![entity_id], |row| {
            Ok(item::EntityLongLong {
                id: row.get(0).unwrap(),
                data_type: row.get(1).unwrap(),
                data: row.get(2).unwrap(),
                last_modified: row.get(3).unwrap(),
            })
        })
        .expect("Something went wrong when querying the database");

    let mut stdout = io::BufWriter::new(io::stdout());
    let row = "-".repeat(78);
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
