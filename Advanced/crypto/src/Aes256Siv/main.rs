use std::fs;
use std::error::Error;

// ============================================================
// APPROACH 1: tink-daead (fully compatible with Mainzelliste)
// Use this if your symmetric_key.der is a Tink JSON keyset.
// ============================================================
mod tink_approach {
    use std::error::Error;
    use tink_core::keyset;
    use tink_core::DeterministicAead;

    pub fn load_and_encrypt(key_path: &str, plaintext: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        tink_daead::init();

        // Read the Tink JSON keyset from file
        let keyset_json = std::fs::read_to_string(key_path)?;
        let cursor = std::io::Cursor::new(keyset_json.as_bytes());

        // Load keyset — use CleartextKeysetHandle for unencrypted keysets
        let kh = keyset::insecure::new_handle(
            tink_core::keyset::JsonReader::new(cursor).read()?
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
            tink_core::keyset::JsonReader::new(cursor).read()?
        )?;

        let daead = tink_daead::new(&kh)?;

        let plaintext = daead.decrypt_deterministically(ciphertext, b"")?;
        Ok(String::from_utf8(plaintext)?)
    }
}

// ============================================================
// APPROACH 2: Raw AES-SIV (lighter, no Tink dependency)
// Use this if you extract the raw key bytes from the keyset.
// NOTE: Tink prepends a 5-byte prefix to ciphertext.
//       This approach skips that — not drop-in compatible
//       unless you handle the prefix manually (see below).
// ============================================================
mod raw_aes_siv {
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let key_path = "symmetric_key.der"; // your Tink JSON keyset file
    let local_id = "HOSP-4567";         // the localid to encrypt

    println!("=== Approach 1: tink-daead (Mainzelliste-compatible) ===");
    let ciphertext = tink_approach::load_and_encrypt(key_path, local_id)?;
    println!("localid:    {}", local_id);
    println!("cryptoid:   {:02x?}", &ciphertext);

    let recovered = tink_approach::load_and_decrypt(key_path, &ciphertext)?;
    println!("decrypted:  {}", recovered);
    assert_eq!(recovered, local_id);

    // Determinism check
    let ct2 = tink_approach::load_and_encrypt(key_path, local_id)?;
    assert_eq!(ciphertext, ct2, "Should be deterministic!");
    println!("Determinism check passed ✓");

    Ok(())
}
