#[cfg(test)]
mod tests {
    use encfs_cracker::crypto::state_encryption;

    #[test]
    fn test_derive_key() {
        let password = "correct horse battery staple";
        let salt = b"static_salt_for_test";
        let key1 = state_encryption::derive_key(password, salt);
        let key2 = state_encryption::derive_key(password, salt);

        assert_eq!(key1.len(), 32);
        assert_eq!(key1, key2);

        let different_password = "wrong horse battery staple";
        let key3 = state_encryption::derive_key(different_password, salt);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let data = b"sensitive fragment data";

        let encrypted = state_encryption::encrypt(data, &key);
        assert_ne!(data.to_vec(), encrypted);

        let decrypted =
            state_encryption::decrypt(&encrypted, &key).expect("Decryption should succeed");
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_decrypt_failure() {
        let key = [0u8; 32];
        let wrong_key = [1u8; 32];
        let data = b"sensitive data";

        let encrypted = state_encryption::encrypt(data, &key);
        let result = state_encryption::decrypt(&encrypted, &wrong_key);

        assert!(result.is_err());
    }
}
