#![allow(unused)]

// pub mod crypto;
mod db;

use db::{DatabaseJob::*, create_db_obj, execute_job};
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let db_path = &args[1];

    let mut db_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(db_path)
        .expect("unable to open database file");
    let mut db = create_db_obj(&mut db_file);

    println!("basic password manager");
    println!("press ctrl+c to quit");

    loop {
        print!("> ");
        io::stdout().flush();

        let mut full_cmd = String::new();
        io::stdin()
            .read_line(&mut full_cmd)
            .expect("unable to read input");
        let full_cmd = full_cmd.trim_end();

        let mut full_cmd = full_cmd.split(" ");
        let cmd_name = full_cmd.next();

        match cmd_name {
            Some(inp) => match inp {
                "help" => println!(
                    "available commands:
insert {{NAME}} {{PASS}}: insert an entry into the database
delete {{NAME}}: delete an existing entry from the database
edit {{NAME}} {{NEW_PASS}}: edit an existing entry
extract {{NAME}}: retrieve an existing entry"
                ),
                "insert" => {
                    let Ok(_) = execute_job(Insert, &mut db, &mut db_file, full_cmd) else {
                        continue;
                    };
                }
                "delete" => {
                    let Ok(_) = execute_job(Delete, &mut db, &mut db_file, full_cmd) else {
                        continue;
                    };
                } // TODO: figure out how to delete only the line in the file containing the entry instead of having to rewrite the whole file
                "edit" => {
                    let Ok(_) = execute_job(Edit, &mut db, &mut db_file, full_cmd) else {
                        continue;
                    };
                } // TODO: figure out how to modify only the line in the file containing the entry without having to rewrite the whole file
                "extract" => {
                    let Ok(_) = execute_job(Extract, &mut db, &mut db_file, full_cmd) else {
                        continue;
                    };
                }
                _ => println!(
                    "unrecognized command: '{inp}'. run the \"help\" command if you don't know what commands are supported."
                ),
            },
            None => println!(
                "invalid input. run the \"help\" command if you don't know what commands are supported."
            ),
        }
    }
}
