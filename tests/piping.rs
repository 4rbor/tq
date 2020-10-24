use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
mod common;

#[test]
fn test_basic_pipe_context() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tq")?;

  cmd
    .arg("--file")
    .arg(common::get_fixture_path("test_01.toml"))
    .arg(". | .cool");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("yes = true"));

  Ok(())
}

#[test]
fn filter_identity_piping_scope() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tq")?;

  cmd
    .arg("--file")
    .arg(common::get_fixture_path("test_01.toml"))
    .arg(". | . | . | .cool");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("yes = true"));

  Ok(())
}

#[test]
fn filter_identity_piping_right_hand() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tq")?;

  cmd
    .arg("--file")
    .arg(common::get_fixture_path("test_01.toml"))
    .arg(". | .cool | .yes");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("true"));

  Ok(())
}
