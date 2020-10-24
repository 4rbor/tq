use std::fs::File;
use std::io::{self, Read};

use crate::errors::*;

pub fn load_toml_from_file(name: &str) -> Result<toml::Value> {
    let mut file = File::open(name).chain_err(|| format!("Failed to open file: {:?}", &name))?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    toml::from_str(&contents).chain_err(|| "File is not valid TOML.")
}

pub fn load_toml_from_stdin() -> Result<toml::Value> {
    let mut content = String::new();
    let _ = io::stdin().lock().read_to_string(&mut content);

    toml::from_str(&content).chain_err(|| "File is not valid TOML.")
}

#[test]
fn load_toml_from_file_without_crash() {
    let _foo = load_toml_from_file("../tests/fixtures/test_01.toml");
}
