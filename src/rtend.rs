use clap::{load_yaml, App};
use std::unreachable;

use rtend::{add, delete, find, list, utils};

fn main() {
    let yml = load_yaml!("rtend/rtend-yaml.yml");
    let matches = App::from_yaml(yml).get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            add::add(add_matches);
        }

        ("delete", Some(delete_matches)) => {
            delete::delete(delete_matches);
        }

        ("find", Some(find_matches)) => {
            find::find(find_matches);
        }

        ("init", Some(_init_matches)) => {
            if utils::check_first_time() {
                utils::create_new_db(true).unwrap();
            } else if !utils::check_database_exists() {
                utils::create_new_db(false).unwrap();
            }
        }

        ("list", Some(list_matches)) => {
            list::list(list_matches);
        }

        ("", None) => println!("Run the program with --help to get started"),
        _ => unreachable!(),
    }
}
