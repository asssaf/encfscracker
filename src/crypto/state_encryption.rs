use anyhow::{anyhow, Result};
use argon2::{Argon2, Params};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{RngExt, rng};

/// Derives a 32-byte key from a password and salt using Argon2id.
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    // Adjust parameters for a balance between security and performance
    // Target: < 500ms startup latency.
    // m_cost: 16MB, t_cost: 1, p_cost: 1
    let params = Params::new(16384, 1, 1, Some(32))
        .expect("Argon2 parameters should be valid");

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params,
    );

    let mut output_key = [0u8; 32];
    let _ = argon2.hash_password_into(password.as_bytes(), salt, &mut output_key);
    output_key
}

/// Encrypts data using AES-256-GCM with a random nonce.
/// The nonce is prepended to the ciphertext.
pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Key should be 32 bytes");
    
    let mut nonce_bytes = [0u8; 12];
    rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .expect("Encryption should succeed");

    let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    result
}

/// Decrypts data using AES-256-GCM.
/// Expects the first 12 bytes to be the nonce.
pub fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 12 {
        return Err(anyhow!("Ciphertext too short"));
    }

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| anyhow!("Invalid key: {}", e))?;
    
    let nonce = Nonce::from_slice(&data[..12]);
    let ciphertext = &data[12..];

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("Decryption failed: {}", e))
}
