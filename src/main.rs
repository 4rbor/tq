#[macro_use]
extern crate clap;
extern crate toml;

use clap::{App, Arg};

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
                .help("TOML file to read")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("PATTERN")
                .help("Field to read from the TOML file")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("file").unwrap();
    println!("Value for config: {}", config);
}
