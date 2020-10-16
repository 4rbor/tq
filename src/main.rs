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

    // .{}

    /***
     * One of the first things we'll want to do is deal with filters
     * and how to pipe results from one filter to another. From the JQ manual:
     * "Generally, things that would be done with loops and iteration in other languages are just done by gluing filters together in jq."
     * The same can be said for tq.
     *
     * It's important to remember that every filter has an input and an output.
     * Even literals like "hello" or 42 are filters - they take an input but
     * always produce the same literal as output. Operations that combine two
     * filters, like addition, generally feed the same input to both and combine
     * the results. So, you can implement an averaging filter as add / length -
     * feeding the input array both to the add filter and the length filter
     * and then performing the division.
     */

    println!("Reading toml file: \n\n{}", toml_file);
    let package = toml_file["package"].as_table().expect("whatever");
    for (key, val) in package {
        println!("key: {}, value: {}", key, val);
    }

    /***
     * Basic filters:
     *
     * Identity: .
     *  - The absolute simplest filter is: .
     *    This is a filter that takes its input and produces it unchanged as output. That is, this is the identity operator.
     *
     * Object Identifier-Index: .foo, .foo.bar
     * - The simplest useful filter is: .foo
     *   When given TOML as an input, it gets the value in the table row "foo"
     */

    let version = toml_file["version"].as_str().expect("whatever");
    println!("version {}", version);
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
