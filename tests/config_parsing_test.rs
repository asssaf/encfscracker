use encfs_cracker::config::EncfsConfig;

#[test]
fn test_parse_valid_config() {
    let xml = r#"
<boost_serialization signature="serialization::archive" version="4">
    <cfg>
        <version>6</version>
        <cipherAlg>aes</cipherAlg>
        <keySize>256</keySize>
        <blockSize>1024</blockSize>
        <uniqueIV>1</uniqueIV>
    </cfg>
</boost_serialization>
"#;
    let config = EncfsConfig::from_xml(xml).expect("Should parse valid xml");
    assert_eq!(config.version, 6);
    assert_eq!(config.cipher_alg, "aes");
}
