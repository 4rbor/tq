#[macro_use]
extern crate clap;
extern crate toml;
#[macro_use]
extern crate error_chain;

/***
 * https://docs.rs/error-chain/0.12.4/error_chain/
 *
* The error_chain! macro declares the types and implementation
* boilerplate necessary for fulfilling a particular error-handling strategy.
* Most importantly it defines a custom error type (called Error
* by convention) and the From conversions that let the ? operator work.
*/
error_chain! {}

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("TOML_FILE")
                .help("TOML file to ingest")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("filter")
                .help("Applies to the TOML input and produces filter results as TOML on standard output.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let toml_file: toml::Value = match matches.value_of("file") {
        Some(file) => load_toml_from_file(file).unwrap(),
        None => load_toml_from_stdn().unwrap(),
    };

    println!("Reading toml file: \n{}", toml_file);
}

fn load_toml_from_file(name: &str) -> Result<toml::Value> {
    let mut file = File::open(name).chain_err(|| format!("Failed to open file: {:?}", &name))?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    toml::from_str(&contents).chain_err(|| "File is not valid TOML.")
}

fn load_toml_from_stdn() -> Result<toml::Value> {
    let mut content = String::new();
    let _ = io::stdin().lock().read_to_string(&mut content);

    toml::from_str(&content).chain_err(|| "File is not valid TOML.")
}
