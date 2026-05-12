use std::process::Command;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_end_to_end_cracker() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.xml");
    let config_content = r#"<boost_serialization>
    <cfg>
        <saltData>SGVsbG8=</saltData>
        <kdfIterations>1000</kdfIterations>
        <keySize>32</keySize>
        <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
    </cfg>
</boost_serialization>"#;
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
