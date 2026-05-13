use quick_xml::de::from_str;
use serde::Deserialize;
use base64::{engine::general_purpose, Engine as _};
use crate::crypto::derive_key;

#[derive(Debug, Deserialize)]
#[serde(rename = "boost_serialization")]
pub struct EncfSConfig {
    pub cfg: ConfigInner,
}

#[derive(Debug, Deserialize)]
pub struct ConfigInner {
    #[serde(rename = "saltData")]
    pub salt_data: String,
    #[serde(rename = "kdfIterations")]
    pub iterations: u32,
    #[serde(rename = "keySize")]
    pub key_size: usize,
    #[serde(rename = "encodedKeyData")]
    pub encoded_key_data: String,
}

impl EncfSConfig {
    pub fn from_xml(xml: &str) -> anyhow::Result<Self> {
        let config: EncfSConfig = from_str(xml)?;
        Ok(config)
    }

    pub fn salt_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let cleaned = self.cfg.salt_data.replace(|c: char| c.is_whitespace(), "");
        Ok(general_purpose::STANDARD.decode(&cleaned)?)
    }

    pub fn encoded_key_data_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let cleaned = self.cfg.encoded_key_data.replace(|c: char| c.is_whitespace(), "");
        Ok(general_purpose::STANDARD.decode(&cleaned)?)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let salt = match self.salt_bytes() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Salt decode error: {}", e);
                return false;
            },
        };
        let encoded_key_data = match self.encoded_key_data_bytes() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Key data decode error: {}", e);
                return false;
            },
        };
        
        // Extract checksum from first 4 bytes
        let mut checksum_bytes = [0u8; 4];
        checksum_bytes.copy_from_slice(&encoded_key_data[0..4]);
        let checksum = u32::from_be_bytes(checksum_bytes);

        let kek = derive_key(password.as_bytes(), &salt, self.cfg.iterations);
        let master_key = &kek[0..32];
        let master_iv = &kek[32..48];
        
        if let Ok(decrypted_key) = crate::crypto::decrypt_encoded_key_data(&encoded_key_data, master_key, master_iv) {
            return crate::crypto::validate_decrypted_key(&decrypted_key, master_key, checksum);
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_encfs_config() {
        let xml = r#"<boost_serialization>
    <cfg>
        <saltData>SGVsbG8=</saltData>
        <kdfIterations>1000</kdfIterations>
        <keySize>32</keySize>
        <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
    </cfg>
</boost_serialization>"#;
        let config = EncfSConfig::from_xml(xml).unwrap();
        assert_eq!(config.salt_bytes().unwrap(), b"Hello");
        assert_eq!(config.cfg.iterations, 1000);
        assert_eq!(config.cfg.key_size, 32);
        assert_eq!(config.encoded_key_data_bytes().unwrap(), b"KeyData");
    }
}
