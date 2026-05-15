use std::process::Command;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_cli_add_fragment() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.xml");
    fs::write(&config_path, "<dummy></dummy>").unwrap();
    let db_path = dir.path().join("test_add.db");

    let output = Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--config")
        .arg(config_path)
        .arg("--db-path")
        .arg(db_path)
        .arg("--add-fragment")
        .arg("secret")
        .arg("--group")
        .arg("A")
        .output()
        .expect("Failed to execute command");

    // This should fail because --add-fragment is not recognized
    assert!(output.status.success(), "CLI should recognize --add-fragment and --group");
}

#[test]
fn test_cli_management_flow() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("management.db");
    
    // 1. Add fragment
    let output = Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--db-path")
        .arg(&db_path)
        .arg("--add-fragment")
        .arg("secret123")
        .arg("--group")
        .arg("G1")
        .output()
        .expect("Failed to add fragment");
    assert!(output.status.success());

    // 2. List fragments
    let output = Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--db-path")
        .arg(&db_path)
        .arg("--list-fragments")
        .output()
        .expect("Failed to list fragments");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("secret123"));
    assert!(stdout.contains("group: G1"));

    // 3. Clear fragments
    let output = Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--db-path")
        .arg(&db_path)
        .arg("--clear-fragments")
        .output()
        .expect("Failed to clear fragments");
    assert!(output.status.success());

    // 4. List again (should be empty)
    let output = Command::new("cargo")
        .env("STATE_PASSWORD", "testpass")
        .arg("run")
        .arg("--")
        .arg("--db-path")
        .arg(&db_path)
        .arg("--list-fragments")
        .output()
        .expect("Failed to list fragments");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No fragments found."));
}
