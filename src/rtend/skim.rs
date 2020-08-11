#![allow(clippy::redundant_closure)]
#![cfg(target_family = "unix")]
use clap::ArgMatches;
use comfy_table::TableComponent::*;
use comfy_table::*;
use comfy_table::{presets, Table};
use regex::Regex;
use skim::prelude::*;
use skim::Skim;
use std::{env, path, process};

use crate::item;

const DEFAULT_DATABSE: &str = "notes";
const DEFAULT_PREVIEW_COMMAND: &str = "list --entity {2} -vv";

struct RtendSkimItem {
    inner: String,
}

impl SkimItem for RtendSkimItem {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(self.inner.as_str().into())
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }
}

fn make_rsi(conn: rusqlite::Connection, term_width: u16) -> RtendSkimItem {
    let mut stmt = conn.prepare("
        SELECT id,
        (SELECT substr(group_concat(name, '; '), 0, 1000) from alias where entity_id = entity.id limit 4) as alias_list,
        (SELECT count(*) from alias where entity_id = entity.id) as alias_count,
        (SELECT count(*) from snippet where entity_id = entity.id) as snippet_count,
        created
        from entity order by 1
        ").expect("Could not prepare stmt");

    let entity_iter = stmt
        .query_map(rusqlite::params![], |row| {
            Ok(item::EntityLong {
                id: row.get(0).unwrap(),
                alias_list: row.get(1).unwrap(),
                alias_count: row.get(2).unwrap(),
                snippet_count: row.get(3).unwrap(),
                created: row.get(4).unwrap(),
            })
        })
        .expect("Could not query map");

    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_NO_BORDERS)
        .remove_style(MiddleIntersections)
        .remove_style(HorizontalLines)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(term_width)
        .set_header(vec![
            Cell::new("ID"),
            Cell::new("Alias List"),
            Cell::new("Aliases"),
            Cell::new("Snippets"),
            Cell::new("Created on"),
        ]);

    for entity in entity_iter {
        let tmp = entity.unwrap();
        table.add_row(vec![
            Cell::new(&tmp.id),
            Cell::new(&tmp.alias_list),
            Cell::new(&tmp.alias_count),
            Cell::new(&tmp.snippet_count),
            Cell::new(&tmp.created.format(time::Format::Rfc3339)),
        ]);
    }

    RtendSkimItem {
        inner: table.to_string(),
    }
}

pub fn skim(args: &ArgMatches, term_width: u16, conn: rusqlite::Connection) {
    let full_preview_command: String;
    let exe_path: path::PathBuf;
    let mut db: &str = DEFAULT_DATABSE;
    let mut preview_command: String = DEFAULT_PREVIEW_COMMAND.to_string();

    if args.is_present("profile") {
        db = args.value_of("profile").unwrap();
        preview_command = format!("--profile {} {}", db, DEFAULT_PREVIEW_COMMAND);
    }

    match env::current_exe() {
        Ok(path) => {
            exe_path = path;
            full_preview_command = format!("{} {}", exe_path.display(), preview_command)
        }
        Err(e) => {
            eprintln!("Could not get the current path, error: {}", e);
            process::exit(1);
        }
    }

    let options = SkimOptionsBuilder::default()
        .preview(Some(&full_preview_command))
        .preview_window(Some("down:50%"))
        .multi(false)
        .header_lines(2)
        .tabstop(Some("4"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for line in make_rsi(conn, term_width).inner.lines() {
        let _ = tx_item.send(Arc::new(RtendSkimItem {
            inner: line.to_string(),
        }));
    }
    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let entry_selected: String;
    let item = selected_items.get(0);
    match item {
        None => {
            process::exit(1);
        }
        Some(item) => entry_selected = item.output().to_string(),
    }

    let re = Regex::new(r"^\s(\d{1,5})").unwrap();
    let regex_entity_id = re.captures(&entry_selected);
    let entity_id;
    match regex_entity_id {
        None => {
            eprintln!("The selected line does not have a valid entity_id");
            process::exit(1);
        }
        Some(capture_groups) => {
            entity_id = capture_groups
                .get(1)
                .expect("Could not get entity_id from capture group 1")
                .as_str()
        }
    };

    process::Command::new(&exe_path)
        .arg("--profile")
        .arg(db)
        .arg("list")
        .arg("-e")
        .arg(entity_id)
        .arg("-vv")
        .status()
        .expect("Could not display result");
}
