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

    let matches = App::from_yaml(yml)
        .template(
            r#"{bin} v{version}
{about}

USAGE:
    {usage}

GLOBAL OPTIONS:
    {options}

SUBCOMMANDS:
    init      Initializes the database
    skim      Skims over the database

------------------------------------------------------------------
    rtend add [OPTIONS] -- Adds new things

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -a, --alias <entity_id> <name>                Adds an alias to an entity
    -e, --entity <name>                           Adds a new entity
    -r, --relation <entity_id_a> <entity_id_b>    Adds a relation between two entities
    -d, --relation-snippet <relation_id>          Adds a snippet to a relation
    -s, --snippet <entity_id>                     Adds a snippet to an entity

------------------------------------------------------------------
    rtend delete [FLAGS] [OPTIONS] -- Deletes things

FLAGS:
    -f, --force    Force delete everything related to the command
                   Use with --entity or --relation

OPTIONS:
    -a, --alias <alias_id>                          Deletes an alias
    -e, --entity <entity_id>                        Deletes an entity
    -r, --relation <relation_id>                    Deletes a relation
    -d, --relation-snippet <relation_snippet_id>    Deletes a relation snippet
    -s, --snippet <snippet_id>                      Deletes a snippet

------------------------------------------------------------------
    rtend edit [OPTIONS] -- Edits things

OPTIONS:
    -a, --alias <alias_id>                          Edits an alias
    -d, --relation-snippet <relation_snippet_id>    Edits a relation snippet
    -s, --snippet <snippet_id>                      Edits a snippet

------------------------------------------------------------------
    rtend find [FLAGS] [OPTIONS] -- Finds thing by its information

FLAGS:
    -v, --verbose    Increases verbosity level
                     Use with --alias or --relation

OPTIONS:
    -a, --alias <name>                 Finds an entity by alias
    -r, --relation <entity_id>         Finds an relation by entity id
    -d, --relation-snippet <string>    Finds an entity by its relation snippet
    -s, --snippet <string>             Finds an entity by its snippet

------------------------------------------------------------------
    rtend list [FLAGS] [OPTIONS] -- Lists information about things

FLAGS:
        --stats      Lists stats about the database
    -v, --verbose    Increases verbosity level
                     Use with --entity or --relation
                     Or as a standalone flag

OPTIONS:
    -a, --alias <entity_id>                 Lists aliases of an entity
    -e, --entity <entity_id>                Lists information about an entity
    -r, --relation <relation_id>            Lists relations of an entity
    -d, --relation-snippet <relation_id>    Lists relation snippets of an entity
    -s, --snippet <entity_id>               Lists snippets of an entity"#,
        )
        .get_matches();

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
    #[cfg(target_family = "unix")]
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
            skim::skim();
        }

        // The program actually never reaches here because of yaml settings
        ("", None) => println!("Run the program with --help to get started"),
        _ => unreachable!(),
    }

    // Windows targets don't get skim feature
    #[cfg(target_family = "windows")]
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
