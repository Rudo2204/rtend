#![cfg(target_family = "unix")]
use regex::Regex;
use skim::{Skim, SkimOptionsBuilder};
use std::{env, io::Cursor, path, process};

static PREVIEW_COMMAND: &str = "list --entity {2} -vv";

pub fn skim() {
    let full_preview_command: String;
    let exe_path: path::PathBuf;

    match env::current_exe() {
        Ok(path) => {
            exe_path = path;
            full_preview_command = format!("{} {}", exe_path.display(), PREVIEW_COMMAND)
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
    let entity_id = re
        .captures(&entry_selected)
        .expect("Could not capture entity_id in output")
        .get(1)
        .expect("Could not get entity_id from capture group 1")
        .as_str();

    process::Command::new(&exe_path)
        .arg("list")
        .arg("-e")
        .arg(entity_id)
        .arg("-vv")
        .status()
        .expect("Could not display result");
}
