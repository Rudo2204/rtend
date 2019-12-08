use clap::{load_yaml, App};
use rusqlite::Connection;
use std::{process, unreachable};

use rtend::{add, delete, find, list, utils};

fn main() {
    let yml = load_yaml!("rtend/rtend-yaml.yml");
    let matches = App::from_yaml(yml).get_matches();

    // First check subcommand init and initialize the database
    if let Some(_init_matches) = matches.subcommand_matches("init") {
        if utils::check_first_time() {
            utils::create_new_db(true).unwrap();
        } else if !utils::check_database_exists() {
            utils::create_new_db(false).unwrap();
        }
    } else if !utils::check_database_exists() {
        eprintln!("database does not exist, please run the subcommand `init`");
        process::exit(1);
    }

    // Future: Change the database `notes.db` accordingly to a config file
    let conn =
        Connection::open(&utils::find_data_dir().unwrap().join("notes.db")).unwrap_or_else(|err| {
            eprintln!("Could not open database! Error: {}", err);
            process::exit(1)
        });

    // Then check every other subcommands
    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            add::add(add_matches, conn);
        }

        ("delete", Some(delete_matches)) => {
            delete::delete(delete_matches, conn);
        }

        ("find", Some(find_matches)) => {
            find::find(find_matches, conn);
        }

        ("list", Some(list_matches)) => {
            list::list(list_matches, conn);
        }

        // The program actually never reaches here because of yaml settings
        ("", None) => println!("Run the program with --help to get started"),
        _ => unreachable!(),
    }
}
