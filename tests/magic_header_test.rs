use encfs_cracker::crypto::validate_decrypted_key;

#[test]
fn test_validate_decrypted_key_valid() {
    let mut data = vec![0x17, 0x7b, 0x25, 0xd9, 0x49, 0x0d, 0xb8, 0xaa];
    data.extend_from_slice(&[0u8; 32]); // add some "key" data
    assert!(validate_decrypted_key(&data));
}

#[test]
fn test_validate_decrypted_key_invalid_header() {
    let mut data = vec![0x00, 0x7b, 0x25, 0xd9, 0x49, 0x0d, 0xb8, 0xaa];
    data.extend_from_slice(&[0u8; 32]);
    assert!(!validate_decrypted_key(&data));
}

#[test]
fn test_validate_decrypted_key_too_short() {
    let data = vec![0x17, 0x7b, 0x25, 0xd9, 0x49, 0x0d, 0xb8];
    assert!(!validate_decrypted_key(&data));
}

#[test]
fn test_validate_decrypted_key_empty() {
    let data = vec![];
    assert!(!validate_decrypted_key(&data));
}
