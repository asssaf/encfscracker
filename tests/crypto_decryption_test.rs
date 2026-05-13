use encfs_cracker::crypto::{decrypt_encoded_key_data, derive_key};

// Define mock parameters for testing decrypt_encoded_key_data.
const MOCK_PASSWORD: &[u8] = b"a_very_secret_password_for_testing_12345"; // 32 bytes
const MOCK_SALT: &[u8] = b"a_random_salt_for_test"; // 16 bytes
const MOCK_ITERATIONS: u32 = 1000;

// The expected decrypted key data length is 48 bytes.
const EXPECTED_DECRYPTED_LEN: usize = 48;

#[test]
fn test_decrypt_encoded_key_data_structure() {
    // Construct dummy encoded_data: 4 bytes checksum + 48 bytes encrypted data.
    let mut encoded_data = vec![0u8; 52]; // 4 bytes for checksum + 48 bytes for encrypted data
    
    // Simulate a checksum (4 bytes).
    encoded_data[0..4].copy_from_slice(&[0x12, 0x34, 0x56, 0x78]); 
    
    // Simulate encrypted data (48 bytes).
    encoded_data[4..52].copy_from_slice(&[1u8; 48]);

    let kek = derive_key(MOCK_PASSWORD, MOCK_SALT, MOCK_ITERATIONS);
    let master_key = &kek[0..32];
    let master_iv = &kek[32..48];

    // Call the function with mock parameters and dummy encoded_data.
    let decrypted_data = decrypt_encoded_key_data(
        encoded_data.as_slice(),
        master_key,
        master_iv
    ).unwrap();

    // Assert that the decrypted data has the expected length.
    assert_eq!(decrypted_data.len(), EXPECTED_DECRYPTED_LEN, "Decrypted data should be {} bytes", EXPECTED_DECRYPTED_LEN);
}
