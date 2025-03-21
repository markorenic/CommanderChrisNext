use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use tempfile::tempdir;

#[test]
fn test_cli_version() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("chris")?;
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    Ok(())
}

#[test]
fn test_cli_help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("chris")?;
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Chris - Your friendly terminal assistant",
    ));
    Ok(())
}

#[test]
fn test_config_create() -> Result<(), Box<dyn Error>> {
    // Create a temporary directory for the config file
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");

    // Run the command to create the config file
    let mut cmd = Command::cargo_bin("chris")?;
    cmd.arg("config")
        .arg("--create")
        .arg("--config")
        .arg(&config_path);

    cmd.assert().success();

    // Verify the config file was created
    assert!(config_path.exists());

    Ok(())
}
