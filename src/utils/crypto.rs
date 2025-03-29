use base64::{engine::general_purpose, Engine as _};
use ring::{aead, digest, pbkdf2, rand};
use std::num::NonZeroU32;
use crate::error::{Result, SecurityError};

// Constants for encryption
const ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };
const CREDENTIAL_LEN: usize = 32;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

/// Derive a key from a password and salt
fn derive_key(password: &[u8], salt: &[u8]) -> [u8; CREDENTIAL_LEN] {
    let mut key = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        ITERATIONS,
        salt,
        password,
        &mut key,
    );
    key
}

/// Encrypt a string with a password
pub fn encrypt(plaintext: &str, password: &str) -> Result<String> {
    // Generate random salt
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; SALT_LEN];
    rng.fill(&mut salt).map_err(|_| SecurityError("Failed to generate salt".into()))?;
    
    // Derive key
    let key = derive_key(password.as_bytes(), &salt);
    let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key)
        .map_err(|_| SecurityError("Failed to create encryption key".into()))?;
    let sealing_key = aead::SealingKey::new(unbound_key);
    
    // Generate nonce
    let mut nonce = [0u8; NONCE_LEN];
    rng.fill(&mut nonce).map_err(|_| SecurityError("Failed to generate nonce".into()))?;
    let nonce = aead::Nonce::assume_unique_for_key(nonce);
    
    // Encrypt
    let mut in_out = plaintext.as_bytes().to_vec();
    let tag = sealing_key.seal_in_place_separate_tag(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|_| SecurityError("Encryption failed".into()))?;
    
    // Concatenate salt + nonce + ciphertext + tag
    let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + in_out.len() + tag.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(nonce.as_ref());
    result.extend_from_slice(&in_out);
    result.extend_from_slice(tag.as_ref());
    
    // Encode as base64
    let encoded = general_purpose::STANDARD.encode(&result);
    Ok(encoded)
}

/// Decrypt a string with a password
pub fn decrypt(ciphertext: &str, password: &str) -> Result<String> {
    // Decode base64
    let data = general_purpose::STANDARD.decode(ciphertext)
        .map_err(|_| SecurityError("Invalid base64 encoding".into()))?;
    
    // Ensure data is long enough
    if data.len() < SALT_LEN + NONCE_LEN + aead::CHACHA20_POLY1305.tag_len() {
        return Err(SecurityError("Invalid ciphertext format".into()));
    }
    
    // Extract components
    let salt = &data[0..SALT_LEN];
    let nonce = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext_and_tag = &data[SALT_LEN + NONCE_LEN..];
    
    // Derive key
    let key = derive_key(password.as_bytes(), salt);
    let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key)
        .map_err(|_| SecurityError("Failed to create decryption key".into()))?;
    let opening_key = aead::OpeningKey::new(unbound_key);
    
    // Set up nonce
    let nonce = aead::Nonce::try_assume_unique_for_key(nonce)
        .map_err(|_| SecurityError("Invalid nonce".into()))?;
    
    // Decrypt
    let mut in_out = ciphertext_and_tag.to_vec();
    let plaintext = opening_key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|_| SecurityError("Decryption failed - invalid password or corrupted data".into()))?;
    
    // Convert to string
    String::from_utf8(plaintext.to_vec())
        .map_err(|_| SecurityError("Decrypted data is not valid UTF-8".into()))
}

/// Hash a string
pub fn hash_string(input: &str) -> String {
    let digest = digest::digest(&digest::SHA256, input.as_bytes());
    general_purpose::STANDARD.encode(digest.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = "This is a secret message";
        let password = "password123";
        
        let encrypted = encrypt(plaintext, password).unwrap();
        let decrypted = decrypt(&encrypted, password).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }
    
    #[test]
    fn test_decrypt_wrong_password() {
        let plaintext = "This is a secret message";
        let password = "password123";
        let wrong_password = "wrong_password";
        
        let encrypted = encrypt(plaintext, password).unwrap();
        let result = decrypt(&encrypted, wrong_password);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_hash_string() {
        let input = "test string";
        let hash1 = hash_string(input);
        let hash2 = hash_string(input);
        
        assert_eq!(hash1, hash2);
    }
} 