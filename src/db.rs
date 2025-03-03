use crate::crypto;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

/// Signature used to signify the start of a .bpmdb file ("startbpmdb")
pub const START_FILE_SIGNATURE: [u8; 10] =
    [0x73, 0x74, 0x61, 0x72, 0x74, 0x62, 0x70, 0x6d, 0x64, 0x62];

/// Signature used to signify the start of a .bpmdb header ("starth")
pub const START_HEADER_SIGNATURE: [u8; 6] = [0x73, 0x74, 0x61, 0x72, 0x74, 0x68];

/// Signature used to signify the end of a .bpmdb header ("endh")
pub const END_HEADER_SIGNATURE: [u8; 4] = [0x65, 0x6e, 0x64, 0x68];

/// Signature used to signify the end of a .bpmdb file ("endbpmdb")
pub const END_FILE_SIGNATURE: [u8; 8] = [0x65, 0x6e, 0x64, 0x62, 0x70, 0x6d, 0x64, 0x62];

pub static OPEN_DB: LazyLock<Option<Database>> = LazyLock::new(|| Option::default());

/// A .bpmdb file.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Database {
    // Path to the database on the user's file system.
    path: PathBuf,

    master_password: String,
    // The unique salt used for the master password in the key derivation function.
    // master_salt: &'a [u8],

    // The nonce generated for encryption.
    // nonce: &'a [u8],

    // The password store itself.
    passwords: HashMap<String, String>,

    /// Database status information
    is_open: bool,
}

// impl Database {
//     pub fn new(path: PathBuf, master_password: String) -> Self {
//         Self {
//             path,
//             master_password,
//             passwords: HashMap::new(),
//         }
//     }
// }

// impl<'a> Database<'a> {
//     pub fn new(password: &'a str) -> Database<'a> {
//         let (key, salt, nonce) = crypto::generate_crypto_inputs(password);

//         Self {
//             master_salt: salt.as_str().as_bytes(),
//             nonce: nonce.as_slice(),
//             passwords: HashMap::new(),
//         }
//     }
// }
