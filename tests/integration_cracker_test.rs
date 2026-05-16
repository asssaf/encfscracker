use std::fs;
use std::process::Command;
use tempfile::tempdir;

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

    let db_path = dir.path().join("cracker_state.db");

    // We don't have a real encfs password to crack, so just run the tool
    // and verify it completes without crashing.
    let output = Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--config")
        .arg(config_path)
        .arg("--fragments")
        .arg("a,b,c")
        .arg("--db-path")
        .arg(db_path)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success(), "Cracker should run successfully");
}

#[test]
fn test_end_to_end_cracker_with_groups() {
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

    let db_path = dir.path().join("group_cracker.db");

    // 1. Add fragments with groups
    Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--db-path")
        .arg(&db_path)
        .arg("--add-fragment")
        .arg("word1")
        .arg("--group")
        .arg("G1")
        .output()
        .unwrap();

    Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--db-path")
        .arg(&db_path)
        .arg("--add-fragment")
        .arg("word2")
        .arg("--group")
        .arg("G1")
        .output()
        .unwrap();

    // 2. Run cracker
    let output = Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--config")
        .arg(config_path)
        .arg("--db-path")
        .arg(db_path)
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Cracker with groups should run successfully"
    );
}
