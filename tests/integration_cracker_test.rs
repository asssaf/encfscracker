use std::process::Command;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_end_to_end_cracker() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.xml");
    let config_content = r#"<config>
    <salt>SGVsbG8=</salt>
    <iterations>1000</iterations>
    <keySize>32</keySize>
    <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
</config>"#;
    fs::write(&config_path, config_content).unwrap();
    
    // We don't have a real encfs password to crack, so just run the tool
    // and verify it completes without crashing.
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--config")
        .arg(config_path)
        .arg("--fragments")
        .arg("a,b,c")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Cracker should run successfully");
}
