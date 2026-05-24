use std::error::Error;
use tink_core::{keyset, DeterministicAead};
use tink_core::keyset::JsonReader;
use tink_core::keyset::Reader;

pub fn load_and_encrypt(key_path: &str, plaintext: &str) -> anyhow::Result<Vec<u8>, Box<dyn Error>> {
    tink_daead::init();

    // Read the Tink JSON keyset from file
    let keyset_json = std::fs::read_to_string(key_path)?;
    let cursor = std::io::Cursor::new(keyset_json.as_bytes());

    // Load keyset — use CleartextKeysetHandle for unencrypted keysets
    let kh = keyset::insecure::new_handle(
        JsonReader::new(cursor).read()?
    )?;

    let daead = tink_daead::new(&kh)?;

    // associated_data is empty in Mainzelliste's usage
    let ciphertext = daead.encrypt_deterministically(plaintext.as_bytes(), b"")?;
    Ok(ciphertext)
}

pub fn load_and_decrypt(key_path: &str, ciphertext: &[u8]) -> Result<String, Box<dyn Error>> {
    tink_daead::init();

    let keyset_json = std::fs::read_to_string(key_path)?;
    let cursor = std::io::Cursor::new(keyset_json.as_bytes());

    let kh = keyset::insecure::new_handle(
        keyset::JsonReader::new(cursor).read()?
    )?;

    let daead = tink_daead::new(&kh)?;

    let plaintext = daead.decrypt_deterministically(ciphertext, b"")?;
    Ok(String::from_utf8(plaintext)?)
}
