use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
mod common;

#[test]
fn file_arg_invalid_location() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;

    cmd.arg("--file")
        .arg(common::get_fixture_path("should-not-exist.toml"))
        .arg(".");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to open file:"));

    Ok(())
}

#[test]
fn file_arg_not_valid_toml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;

    cmd.arg("--file")
        .arg(common::get_fixture_path("test_01.json"))
        .arg(".");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File is not valid TOML."));

    Ok(())
}
