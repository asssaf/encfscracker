use encfs_cracker::crypto::{validate_decrypted_key, HmacSha1};
use hmac::Mac;

// Helper to calculate checksum for testing purposes
fn calculate_checksum(decrypted_data: &[u8], master_key: &[u8]) -> u32 {
    let mut mac = HmacSha1::new_from_slice(master_key).unwrap();
    mac.update(decrypted_data);
    let result = mac.finalize().into_bytes();

    let mut h = [0u8; 8];
    for i in 0..19 {
        h[i % 8] ^= result[i];
    }

    let mut mac64: u64 = 0;
    for &byte in &h {
        mac64 = (mac64 << 8) | (byte as u64);
    }
    ((mac64 >> 32) as u32) ^ (mac64 as u32)
}

#[test]
fn test_validate_decrypted_key_valid() {
    let master_key_bytes: [u8; 32] = [1u8; 32];
    let decrypted_data_bytes: [u8; 48] = [2u8; 48];
    let expected_checksum = calculate_checksum(&decrypted_data_bytes, &master_key_bytes);

    assert!(validate_decrypted_key(
        &decrypted_data_bytes,
        &master_key_bytes,
        expected_checksum
    ));
}

#[test]
fn test_validate_decrypted_key_invalid_checksum() {
    let master_key_bytes: [u8; 32] = [1u8; 32];
    let decrypted_data_bytes: [u8; 48] = [2u8; 48];
    let correct_checksum = calculate_checksum(&decrypted_data_bytes, &master_key_bytes);
    let incorrect_checksum = correct_checksum.wrapping_add(1);

    assert!(!validate_decrypted_key(
        &decrypted_data_bytes,
        &master_key_bytes,
        incorrect_checksum
    ));
}

#[test]
fn test_validate_decrypted_key_too_short_data() {
    let master_key_bytes: [u8; 32] = [1u8; 32];
    let decrypted_data_bytes: [u8; 10] = [1u8; 10];
    let expected_checksum = 0;

    assert!(!validate_decrypted_key(
        &decrypted_data_bytes,
        &master_key_bytes,
        expected_checksum
    ));
}

#[test]
fn test_validate_decrypted_key_empty_data() {
    let master_key_bytes: [u8; 32] = [1u8; 32];
    let decrypted_data_bytes: [u8; 0] = [];
    let expected_checksum = 0;

    assert!(!validate_decrypted_key(
        &decrypted_data_bytes,
        &master_key_bytes,
        expected_checksum
    ));
}
