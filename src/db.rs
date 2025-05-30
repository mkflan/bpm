use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
};

/// Create a database object from the database file.
pub fn create_db_obj(db_file: &mut File) -> Database {
    let mut db = Database::new();

    let mut entries = String::new();
    db_file
        .read_to_string(&mut entries)
        .expect("unable to read from file");

    // Entries in the database text file are each one line long. The different parts of an entry
    // are separated by a single space: the left is the entry name, the right is the password.
    for entry in entries.lines() {
        let Some((name, password)) = entry.split_once(" ") else {
            panic!("unable to parse entry")
        };

        db.insert_entry(name.into(), password.into());
    }

    db
}

#[derive(Debug, Default)]
pub struct Database {
    /// The database entries.
    entries: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a new entry into the database.
    pub fn insert_entry(&mut self, name: String, password: String) {
        self.entries.insert(name, password);
    }

    /// Delete an existing entry from the database.
    pub fn delete_entry(&mut self, name: String) {
        self.entries.remove(&name);
    }

    /// Edit an existing entry in the database.
    pub fn edit_entry(&mut self, name: String, new_password: String) {
        self.entries.entry(name).and_modify(|e| *e = new_password);
    }

    /// Extract the password of an existing entry.
    pub fn extract_entry_password(&mut self, name: String) -> &String {
        self.entries.get(&name).expect("unable to retrieve entry")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseJob {
    Insert,
    Delete,
    Edit,
    Extract,
}

pub fn execute_job<'a>(
    job: DatabaseJob,
    db: &mut Database,
    db_file: &mut File,
    mut args: impl Iterator<Item = &'a str>,
) -> Result<(), ()> {
    use DatabaseJob::*;

    match job {
        Insert => {
            let Some(entry_name) = args.next() else {
                println!("Please provide a name for this entry!");
                return Err(());
            };

            let Some(password) = args.next() else {
                println!("Please provide a password to accompany this entry!");
                return Err(());
            };

            db.insert_entry(entry_name.into(), password.into());
            db_file.write(format!("{entry_name} {password}\n").as_bytes());

            println!("Successfully created entry for '{entry_name}'!");
        }
        Delete => todo!(),
        Edit => todo!(),
        Extract => todo!(),
    }

    Ok(())
}
