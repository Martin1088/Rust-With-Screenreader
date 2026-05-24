use aes_siv::{
    Aes256SivAead,
    aead::{KeyInit, Aead, generic_array::GenericArray},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use std::error::Error;

// The Tink TINK output prefix: 1 byte (0x01) + 4 bytes key ID
// Mainzelliste uses outputPrefixType = "TINK", so ciphertexts
// from Mainzelliste will have this 5-byte prefix prepended.
const TINK_PREFIX_SIZE: usize = 5;

/// Load raw AES-256-SIV key bytes from a base64 string
/// (extracted from the "value" field of the Tink JSON keyset).
/// The Tink AesSivKey proto encodes: 2 bytes tag + 64 bytes key
/// So we skip the first 2 bytes to get the raw key.
pub fn key_from_tink_value(base64_value: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded = STANDARD.decode(base64_value)?;
    // Skip 2-byte protobuf tag to get raw 64-byte AES-SIV key
    Ok(decoded[2..].to_vec())
}

pub fn encrypt(raw_key: &[u8], plaintext: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let key = GenericArray::from_slice(raw_key);
    let cipher = Aes256SivAead::new(key);
    // AES-SIV with empty nonce (deterministic)
    let ct = cipher.encrypt(GenericArray::from_slice(&[0u8; 16]), plaintext.as_bytes())
        .map_err(|e| format!("encrypt error: {e}"))?;
    Ok(ct)
}

pub fn decrypt(raw_key: &[u8], ciphertext_with_prefix: &[u8]) -> Result<String, Box<dyn Error>> {
    // Strip the 5-byte Tink prefix if present
    let ciphertext = if ciphertext_with_prefix.len() > TINK_PREFIX_SIZE
        && ciphertext_with_prefix[0] == 0x01
    {
        &ciphertext_with_prefix[TINK_PREFIX_SIZE..]
    } else {
        ciphertext_with_prefix
    };

    let key = GenericArray::from_slice(raw_key);
    let cipher = Aes256SivAead::new(key);
    let pt = cipher.decrypt(GenericArray::from_slice(&[0u8; 16]), ciphertext)
        .map_err(|e| format!("decrypt error: {e}"))?;
    Ok(String::from_utf8(pt)?)
}
