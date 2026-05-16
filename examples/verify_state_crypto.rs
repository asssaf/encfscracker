use encfs_cracker::crypto::state_encryption;

fn main() -> anyhow::Result<()> {
    let password = "my_secure_password";
    let salt = b"unique_salt_123";

    println!("--- Phase 1: State Encryption Verification ---");

    // 1. Key Derivation
    println!("Deriving key from password...");
    let key = state_encryption::derive_key(password, salt);
    println!("Derived key: {:?}", key);

    // 2. Encryption
    let data = b"This is a secret fragment: 'P@ssw0rd123'";
    println!("Original data: {:?}", String::from_utf8_lossy(data));

    let encrypted = state_encryption::encrypt(data, &key);
    println!("Encrypted data: {:?}", encrypted);

    // 3. Decryption
    println!("Decrypting...");
    let decrypted = state_encryption::decrypt(&encrypted, &key)?;
    println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));

    assert_eq!(data.to_vec(), decrypted);
    println!("Verification SUCCESS: Data matches perfectly.");

    Ok(())
}
