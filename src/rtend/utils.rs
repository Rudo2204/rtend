use directories::ProjectDirs;
use rusqlite;
use rusqlite::{Connection, NO_PARAMS};
use std::{
    convert::TryInto,
    fs,
    io::{self, Write},
    path, process,
};

const PROGRAM_NAME: &str = "rtend";

pub fn get_yn_input() -> Result<bool, ()> {
    let answer;
    let yes = vec!["y", "Y", "yes", "YES", "Yes"];
    let no = vec!["n", "N", "no", "NO", "No"];
    print!("Proceed? [y/n]: ");
    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => eprintln!("Could not read stdin, error: {}", e),
        }
        input = input.trim().to_string();
        if yes.iter().any(|n| &n[..] == input) {
            answer = true;
            break;
        } else if no.iter().any(|n| &n[..] == input) {
            answer = false;
            break;
        }
        eprintln!("Invalid input, exiting");
        process::exit(1);
    }
    Ok(answer)
}

pub fn get_term_width() -> u16 {
    if let Some((w, _)) = term_size::dimensions() {
        w.try_into().unwrap()
    } else {
        eprintln!("Unable to get terminal size!");
        process::exit(1);
    }
}

pub fn check_database_exists(name: &str) -> bool {
    find_data_dir().unwrap().join(name).exists()
}

pub fn trim_trailing_newline(s: &mut String) -> String {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop().unwrap();
    }

    s.to_string()
}

pub fn check_first_time() -> bool {
    let rtend_data_dir = find_data_dir().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    !rtend_data_dir.exists()
}

pub fn find_data_dir() -> Result<path::PathBuf, &'static str> {
    if let Some(base_dir) = ProjectDirs::from("", "", PROGRAM_NAME) {
        Ok(base_dir.data_dir().to_path_buf())
    } else {
        Err("Could not retrieve home directory. You maybe are using unsupported OS.")
    }
}

pub fn create_new_db(first_time: bool, name: &str) -> rusqlite::Result<()> {
    let rtend_data_dir = find_data_dir().unwrap();

    println!(
        "rtend's data does not exist, will now create one at: {}",
        rtend_data_dir.display()
    );

    if first_time {
        match fs::create_dir(&rtend_data_dir) {
            Ok(()) => (),
            Err(err) => eprintln!("Problem creating data directory: {}", err),
        }
    }

    let conn = Connection::open(&rtend_data_dir.join(name))?;

    // Importing schema
    conn.execute(
        "CREATE TABLE entity (
             id integer primary key autoincrement,
             created datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE snippet (
            id integer primary key,
            entity_id integer references entity(id),
            data text not null,
            created datetime not null default current_timestamp,
            updated datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE alias (
            id integer primary key,
            entity_id integer references entity(id),
            name varchar(255) not null,
            created datetime not null default current_timestamp,
            updated datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE relation (
            id integer primary key,
            entity_id_a integer not null references entity(id),
            entity_id_b integer not null references entity(id),
            created datetime not null default current_timestamp,
            updated datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE relation_snippet (
            id integer primary key,
            relation_id integer not null references relation(id),
            data text not null,
            created datetime not null default current_timestamp,
            updated datetime not null default current_timestamp
         )",
        NO_PARAMS,
    )?;

    println!("Sucessully created database.");
    Ok(())
}
