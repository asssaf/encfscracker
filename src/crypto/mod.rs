pub mod encfs_config;

use pbkdf2::pbkdf2;
use sha2::Sha256;
type HmacSha256 = hmac::Hmac<Sha256>;

pub fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> Vec<u8> {
    let mut derived_key = vec![0u8; 32];
    let _ = pbkdf2::<HmacSha256>(password, salt, iterations, &mut derived_key);
    derived_key
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
