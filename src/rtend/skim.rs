#![cfg(target_family = "unix")]
use clap::ArgMatches;
use regex::Regex;
use skim::{Skim, SkimOptionsBuilder};
use std::{env, io::Cursor, path, process};

const DEFAULT_DATABSE: &str = "notes";
const DEFAULT_PREVIEW_COMMAND: &str = "list --entity {2} -vv";

pub fn skim(args: &ArgMatches) {
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

    let buf = process::Command::new(&exe_path)
        .arg("--profile")
        .arg(db)
        .arg("list")
        .arg("-v")
        .output()
        .unwrap()
        .stdout;

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(buf))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let entry_selected: String;
    let item = selected_items.iter().nth(0);
    match item {
        None => {
            process::exit(1);
        }
        Some(item) => entry_selected = item.get_output_text().to_string(),
    }

    let re = Regex::new(r"^\s?(\d{1,5})").unwrap();
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
