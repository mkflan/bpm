use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit,
    aead::{Aead, AeadMutInPlace, generic_array::GenericArray},
};
use argon2::{
    Argon2,
    password_hash::{SaltString, rand_core::OsRng},
};

/// Generate an AES-256 key from the database password and a nonce for encryption.
pub fn generate_crypto_inputs(
    password: &str,
) -> (
    Vec<u8>,
    SaltString,
    GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize>,
) {
    let argon_ctx = Argon2::default();

    let mut aes_key = vec![0u8; 32]; // AES-256 uses 256-bit keys.
    let salt = SaltString::generate(&mut OsRng);

    argon_ctx
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut aes_key)
        .expect("failed to derive key");

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    (aes_key, salt, nonce)
}

/// Based on the given password, derive a AES-256 key from it.
pub fn derive_key(password: &str) -> Key<Aes256Gcm> {
    let argon_ctx = Argon2::default();
    let mut key = vec![0u8; 32]; // AES-256 uses 256-bit keys.
    let salt = SaltString::generate(&mut OsRng);

    argon_ctx
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut key)
        .expect("failed to derive key");

    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(&key);

    *key
}

/// Encrypt the password database.
pub fn encrypt_db(
    key: Key<Aes256Gcm>,
    nonce: GenericArray<
        u8,
        aes_gcm::aes::cipher::typenum::UInt<
            aes_gcm::aes::cipher::typenum::UInt<
                aes_gcm::aes::cipher::typenum::UInt<
                    aes_gcm::aes::cipher::typenum::UInt<
                        aes_gcm::aes::cipher::typenum::UTerm,
                        aes_gcm::aead::consts::B1,
                    >,
                    aes_gcm::aead::consts::B1,
                >,
                aes_gcm::aead::consts::B0,
            >,
            aes_gcm::aead::consts::B0,
        >,
    >,
    plaintext: &[u8],
) -> Vec<u8> {
    let mut cipher = Aes256Gcm::new(&key);

    cipher
        .encrypt(&nonce, plaintext)
        .expect("unable to encrypt")
}

/// Decrypt the password database.
pub fn decrypt_db(
    key: Key<Aes256Gcm>,
    nonce: GenericArray<
        u8,
        aes_gcm::aes::cipher::typenum::UInt<
            aes_gcm::aes::cipher::typenum::UInt<
                aes_gcm::aes::cipher::typenum::UInt<
                    aes_gcm::aes::cipher::typenum::UInt<
                        aes_gcm::aes::cipher::typenum::UTerm,
                        aes_gcm::aead::consts::B1,
                    >,
                    aes_gcm::aead::consts::B1,
                >,
                aes_gcm::aead::consts::B0,
            >,
            aes_gcm::aead::consts::B0,
        >,
    >,
    ciphertext: &[u8],
) -> Vec<u8> {
    let mut cipher = Aes256Gcm::new(&key);

    match cipher.decrypt(&nonce, ciphertext) {
        Ok(plaintext) => {
            for byte in &plaintext {
                println!("{byte:#X}");
            }

            plaintext
        }
        Err(err) => panic!("error: {err}"),
    }
}
