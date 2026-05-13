use hmac::{Hmac, Mac};
use sha1::Sha1;
use pbkdf2::pbkdf2;
use aes::Aes256;
use cfb_mode::Decryptor;
// Import AsyncStreamCipher for decrypt method
use aes::cipher::{KeyIvInit, AsyncStreamCipher}; 
use anyhow::{anyhow, Result};
use digest::Digest; // Import Digest trait

// Declare the encfs_config module as public and re-export EncfSConfig publicly
pub mod encfs_config; // Make the module public
pub use encfs_config::EncfSConfig; // Publicly re-export EncfSConfig

// Make HmacSha1 public
pub type HmacSha1 = Hmac<Sha1>;
type Aes256CfbDec = Decryptor<Aes256>;

pub fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> Vec<u8> {
    let mut derived_key = vec![0u8; 32 + 16]; // Key (32) + IV (16)
    // Handle the Result returned by pbkdf2
    let _ = pbkdf2::<HmacSha1>(password, salt, iterations, &mut derived_key);
    derived_key
}

pub fn set_ivec(master_key: &[u8], master_iv: &[u8], seed: u64) -> [u8; 16] {
    let mut mac = HmacSha1::new_from_slice(master_key).unwrap();
    mac.update(master_iv);
    let mut seed_bytes = [0u8; 8];
    for i in 0..8 {
        seed_bytes[i] = (seed >> (i * 8)) as u8;
    }
    mac.update(&seed_bytes);
    let result = mac.finalize().into_bytes();
    let mut ivec = [0u8; 16];
    ivec.copy_from_slice(&result[0..16]);
    ivec
}

pub fn unshuffle_bytes(buf: &mut [u8]) {
    for i in (1..buf.len()).rev() {
        buf[i] ^= buf[i - 1];
    }
}

pub fn shuffle_bytes(buf: &mut [u8]) {
    for i in 0..buf.len() - 1 {
        buf[i + 1] ^= buf[i];
    }
}

pub fn flip_bytes(buf: &mut [u8]) {
    buf.reverse();
}

pub fn decrypt_encoded_key_data(encoded_data: &[u8], iterations: u32, salt: &[u8], password: &[u8]) -> Result<Vec<u8>> {
    if encoded_data.len() < 52 {
        return Err(anyhow!("Encoded data too short"));
    }
    let master_key_iv = derive_key(password, salt, iterations);
    let master_key = &master_key_iv[0..32];
    let master_iv = &master_key_iv[32..48];

    // The first 4 bytes of encoded_data are the checksum. The rest is encrypted.
    let checksum = u32::from_be_bytes(encoded_data[0..4].try_into()?);
    let encrypted_payload = &encoded_data[4..52]; // 48 bytes

    let mut buf = encrypted_payload.to_vec();

    // Pass 1
    let ivec1 = set_ivec(master_key, master_iv, (checksum as u64) + 1);
    // The decrypt method is now available because AsyncStreamCipher trait is in scope
    Aes256CfbDec::new(master_key.into(), ivec1.as_slice().into()).decrypt(&mut buf);
    unshuffle_bytes(&mut buf);
    flip_bytes(&mut buf);

    // Pass 2
    let ivec2 = set_ivec(master_key, master_iv, checksum as u64);
    // The decrypt method is now available because AsyncStreamCipher trait is in scope
    Aes256CfbDec::new(master_key.into(), ivec2.as_slice().into()).decrypt(&mut buf);
    unshuffle_bytes(&mut buf);

    Ok(buf)
}

pub fn validate_decrypted_key(decrypted_data: &[u8], master_key: &[u8], expected_checksum: u32) -> bool {
    let mut mac = HmacSha1::new_from_slice(master_key).unwrap();
    mac.update(decrypted_data);
    let result = mac.finalize().into_bytes();
    
    let mut h = [0u8; 8];
    // The loop in validate_decrypted_key goes from 0 to 18 (inclusive).
    // It accesses result[i] where i is up to 18. HMAC-SHA1 produces 20 bytes.
    // So, result must be at least 19 bytes long for this loop to not panic on index out of bounds.
    for i in 0..19 {
        h[i % 8] ^= result[i];
    }

    let mut mac64: u64 = 0;
    for &byte in &h {
        mac64 = (mac64 << 8) | (byte as u64);
    }
    let mac32 = ((mac64 >> 32) as u32) ^ (mac64 as u32);
    
    mac32 == expected_checksum
}
