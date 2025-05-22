#![allow(unused)]

// pub mod crypto;
// mod db;

use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let db_path = &args[1];

    let db_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(db_path)
        .expect("unable to open database file");

    println!("basic password manager");

    loop {
        print!("> ");
        io::stdout().flush();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("unable to read input");

        // match command.split_once(" ") {
        //     Some((cmd, args)) => match cmd {
        //         "help" => todo!(),
        //         "insert" => todo!(),
        //         "delete" => todo!(),
        //         "edit" => todo!(),
        //         "extract" => todo!(),
        //         _ => panic!("invalid command: {command}"),
        //     },
        //     None => panic!("unrecognized input: {command}"),
        // }
    }
}
