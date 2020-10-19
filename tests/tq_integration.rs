use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::io::prelude::*;
use std::process::{Command, Stdio}; // Run programs

fn get_fixture_path(file_name: &str) -> String {
  format!(
    "{project_root}/tests/fixtures/{file_name}",
    project_root = env!("CARGO_MANIFEST_DIR"),
    file_name = file_name
  )
}

// * Identity: .
// *  - The absolute simplest filter is: .
// *    This is a filter that takes its input and produces it unchanged as output. That is, this is the identity operator.
// *
#[test]
fn test_identity_filter_file_arg() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tq")?;

  cmd
    .arg("--file")
    .arg(get_fixture_path("test_01.toml"))
    .arg(".");

  cmd.assert().success().stdout(predicate::str::contains(
    "[cool]\n\
    yes = true",
  ));

  Ok(())
}

#[test]
fn test_identity_filter_stdin() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tq")?;

  let process = match cmd
    .arg(".")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
  {
    Err(why) => panic!("couldn't spawn tq: {}", why),
    Ok(process) => process,
  };

  let toml_str = std::fs::read_to_string(get_fixture_path("test_01.toml"))?;
  let toml_bytes = toml_str.into_bytes();

  match process.stdin.unwrap().write_all(&toml_bytes) {
    Err(why) => panic!("couldn't write to tq stdin: {}", why),
    Ok(_) => println!("sent toml bytes to tq"),
  }

  let mut s = String::new();
  match process.stdout.unwrap().read_to_string(&mut s) {
    Err(why) => panic!("couldn't read tq stdout: {}", why),
    Ok(_) => print!("tq responded with:\n{}", s),
  }

  assert_eq!(
    s,
    "[cool]\n\
  yes = true\n\n"
  );

  Ok(())
}

// TODO: Test if "--file" is provided but file does not exist
