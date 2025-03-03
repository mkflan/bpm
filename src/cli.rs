use crate::{
    crypto::{decrypt_db, encrypt_db, generate_crypto_inputs},
    db::{
        Database, END_FILE_SIGNATURE, END_HEADER_SIGNATURE, OPEN_DB, START_FILE_SIGNATURE,
        START_HEADER_SIGNATURE,
    },
};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use clap::{Args, Parser, Subcommand};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

const KEY_SIZE_BYTES: usize = 32;
const NONCE_SIZE_BYTES: usize = 12;

/// Create a new password database.
#[derive(Args)]
pub struct Create {
    /// The name of the database.
    name: String,

    /// The database's master password.
    #[arg(short)]
    password: String,

    /// The directory in which to create the database.
    #[arg(short, default_value = ".")]
    dir: PathBuf,
}

/// Open an existing password database.
#[derive(Args)]
pub struct Open {
    /// Path to the database to open.
    database: PathBuf,

    /// Master password of the database.
    #[arg(short)]
    password: String,
}

/// Add a new entry to a password database.
#[derive(Args)]
pub struct Add {
    database: PathBuf,
    service: String,
    password: String,
}

/// Remove an entry from a password database.
#[derive(Args)]
pub struct Remove {
    database: PathBuf,
    service: String,
}

/// Retrieve an entry from a password database.
#[derive(Args)]
pub struct Retrieve {
    database: PathBuf,
    service: String,
}

/// Modify an existing entry in a password database.
#[derive(Args)]
pub struct Modify {
    database: PathBuf,
    service: String,
    new_password: String,
}

#[derive(Subcommand)]
pub enum Command {
    Create(Create),
    Open(Open),
    Add(Add),
    Remove(Remove),
    Retrieve(Retrieve),
    Modify(Modify),
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub fn handle_create(args: Create) {
    let mut path = args.dir;
    path.push(args.name);

    let mut db = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)
        .expect("unable to open file");

    let mut plaintext = Vec::<u8>::new();

    // Write start signatures.
    plaintext.extend_from_slice(&START_FILE_SIGNATURE);
    plaintext.extend_from_slice(&START_HEADER_SIGNATURE);

    // do crypto stuff, write header

    // Write header end signature.
    plaintext.extend_from_slice(&END_HEADER_SIGNATURE);

    // actual database entries here

    // Write file end signature.
    plaintext.extend_from_slice(&END_FILE_SIGNATURE);

    // encrypt
    let (key, salt, nonce) = generate_crypto_inputs(&args.password);
    let key = Key::<Aes256Gcm>::from_slice(&key);

    let mut ciphertext = encrypt_db(*key, nonce, &plaintext);

    // TODO: how to store this in the encrypted file and get it back easily. how to store key and nonce better for easy access while decrypting
    // ciphertext.extend_from_slice(&key);
    // ciphertext.extend_from_slice(&nonce);

    db.write_all(&ciphertext).expect("unable to write");
}

pub fn handle_open(args: Open) {
    let db_path = args.database;
    let ciphertext = fs::read(db_path).expect("unable to read from file");

    let key = Key::<Aes256Gcm>::from_slice(
        &ciphertext[(ciphertext.len() - NONCE_SIZE_BYTES - KEY_SIZE_BYTES)..],
    );
    let nonce = Nonce::from_slice(&ciphertext[(ciphertext.len() - NONCE_SIZE_BYTES)..]);
    let plaintext = decrypt_db(*key, *nonce, &ciphertext);
    assert_eq!(&plaintext[..10], &START_FILE_SIGNATURE, "not a .bpmdb file");
    // TODO: deserialize, get stuff needed for decryption, then decrypt

    // let db_master_password = args.password;
    // let db: Database;
    // let db = Database::new(db_path, db_master_password);
}
