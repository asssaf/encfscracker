use encfs_cracker::crypto::encfs_config::EncfSConfig;
use encfs_cracker::config::CrackerConfig;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let xml = r#"<config>
    <salt>SGVsbG8=</salt>
    <iterations>1000</iterations>
    <keySize>32</keySize>
    <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
</config>"#;
    let encfs_cfg = EncfSConfig::from_xml(xml)?;
    let cracker_cfg = CrackerConfig {
        fragments: vec!["a".to_string(), "b".to_string()],
        encfs_config: encfs_cfg,
        db_path: PathBuf::from("state.db"),
    };
    println!("Successfully loaded config: {:?}", cracker_cfg.fragments);
    println!("Salt bytes: {:?}", cracker_cfg.encfs_config.salt_bytes()?);
    Ok(())
}
