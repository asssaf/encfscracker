pub mod encfs_config;

use pbkdf2::pbkdf2;
use sha1::Sha1;
type HmacSha1 = hmac::Hmac<Sha1>;

pub fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> Vec<u8> {
    let mut derived_key = vec![0u8; 32];
    let _ = pbkdf2::<HmacSha1>(password, salt, iterations, &mut derived_key);
    derived_key
}

use aes::Aes256;
use cfb_mode::Decryptor;
use aes::cipher::{KeyIvInit, AsyncStreamCipher};

type Aes256CfbDec = Decryptor<Aes256>;

pub fn decrypt_encoded_key_data(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut buffer = ciphertext.to_vec();
    Aes256CfbDec::new(key.into(), iv.into()).decrypt(&mut buffer);
    Ok(buffer)
}

pub fn validate_decrypted_key(data: &[u8]) -> bool {
    const MAGIC_HEADER: [u8; 8] = [0x17, 0x7b, 0x25, 0xd9, 0x49, 0x0d, 0xb8, 0xaa];
    data.len() >= 8 && data[0..8] == MAGIC_HEADER
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key() {
        let password = b"password";
        let salt = b"salt";
        let iterations = 1000;
        let derived_key = derive_key(password, salt, iterations);
        assert_eq!(derived_key.len(), 32);
    }
}
