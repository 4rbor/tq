#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

use clap::{App, Arg, ArgMatches};

pub mod errors {
    error_chain! {}
}

pub mod filter;
pub mod load_toml;

pub fn read_args() -> ArgMatches<'static> {
    App::new(crate_name!())
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
            .get_matches()
}
