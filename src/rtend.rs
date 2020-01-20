use clap::{load_yaml, App};
use rusqlite::Connection;
use std::{process, unreachable};

#[cfg(target_family = "unix")]
use rtend::{add, delete, edit, find, list, skim, utils};

#[cfg(target_family = "windows")]
use rtend::{add, delete, edit, find, list, utils};

fn main() {
    #[cfg(target_family = "unix")]
    let yml = load_yaml!("rtend/rtend-yaml.yml");
    #[cfg(target_family = "windows")]
    let yml = load_yaml!("rtend/rtend-yaml-windows.yml");

    let matches = App::from_yaml(yml).get_matches();

    // By default the program operates on the database `notes.db`
    // It would switch to whatever database if user uses the --profile flag
    let mut db = "notes.db".to_string();
    if matches.is_present("profile") {
        db = format!("{}.db", matches.value_of("profile").unwrap());
    }

    // First check if the database exists yet, if not then would prompt the user to init it first
    if let Some(_init_matches) = matches.subcommand_matches("init") {
        if utils::check_first_time() {
            utils::create_new_db(true, &db).unwrap();
        } else if !utils::check_database_exists(&db) {
            utils::create_new_db(false, &db).unwrap();
        }
    } else if !utils::check_database_exists(&db) {
        eprintln!("database does not exist, please run the subcommand `init`");
        process::exit(1);
    }

    let conn = Connection::open(&utils::find_data_dir().unwrap().join(&db)).unwrap_or_else(|err| {
        eprintln!("Could not open database! Error: {}", err);
        process::exit(1)
    });

    // Then check every other subcommands
    if cfg!(target_family = "unix") {
        match matches.subcommand() {
            ("add", Some(add_matches)) => {
                add::add(add_matches, conn);
            }

            ("delete", Some(delete_matches)) => {
                delete::delete(delete_matches, conn);
            }

            ("edit", Some(edit_matches)) => {
                edit::edit(edit_matches, conn);
            }

            ("find", Some(find_matches)) => {
                find::find(find_matches, conn);
            }

            // It was already hanlded in the above code, it still needs to be here though
            // else the program would panic because of unreachable code
            ("init", Some(_init_matches)) => {}

            ("list", Some(list_matches)) => {
                list::list(list_matches, conn);
            }

            ("skim", Some(_skim_matches)) => {
                skim::skim(conn);
            }

            // The program actually never reaches here because of yaml settings
            ("", None) => println!("Run the program with --help to get started"),
            _ => unreachable!(),
        }
    // Windows targets don't get skim feature
    } else {
        match matches.subcommand() {
            ("add", Some(add_matches)) => {
                add::add(add_matches, conn);
            }

            ("delete", Some(delete_matches)) => {
                delete::delete(delete_matches, conn);
            }

            ("edit", Some(edit_matches)) => {
                edit::edit(edit_matches, conn);
            }

            ("find", Some(find_matches)) => {
                find::find(find_matches, conn);
            }

            // It was already hanlded in the above code, it still needs to be here though
            // else the program would panic because of unreachable code
            ("init", Some(_init_matches)) => {}

            ("list", Some(list_matches)) => {
                list::list(list_matches, conn);
            }

            // The program actually never reaches here because of yaml settings
            ("", None) => println!("Run the program with --help to get started"),
            _ => unreachable!(),
        }
    }
}
