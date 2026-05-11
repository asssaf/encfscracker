use serde::Deserialize;
use quick_xml::de::from_str;

#[derive(Debug, Deserialize, PartialEq)]
pub struct EncfsConfig {
    pub version: i32,
    #[serde(rename = "cipherAlg")]
    pub cipher_alg: String,
    #[serde(rename = "keySize")]
    pub key_size: i32,
    #[serde(rename = "blockSize")]
    pub block_size: i32,
    #[serde(rename = "uniqueIV")]
    pub unique_iv: i32,
}

#[derive(Debug, Deserialize)]
struct BoostSerialization {
    #[serde(rename = "cfg")]
    pub cfg: EncfsConfig,
}

impl EncfsConfig {
    pub fn from_xml(xml: &str) -> anyhow::Result<Self> {
        let wrapper: BoostSerialization = from_str(xml)?;
        Ok(wrapper.cfg)
    }
}
