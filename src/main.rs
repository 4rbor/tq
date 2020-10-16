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
     *
     * Optional Object Identifier Index: .foo?
     * - Just like .foo, but does not output even an error when . is not an array or an object.
     *
     * Generic Object Index: .[<string>]
     * - You can also look up fields of an object using syntax like .["foo"]
     *   (.foo above is a shorthand version of this, but only for identifier-like strings).
     *
     * Array Index: .[2]
     * When the index value is an integer, .[<value>] can index arrays. Arrays are zero-based, so .[2] returns the third element.
     *
     * Pipe: |
     * Combines two filters by feeding the output(s) of the one on the left into
     * the input of the one on the right. It's pretty much the same as the Unix shell's
     * pipe, if you're used to that. If the one on the left produces multiple results,
     * the one on the right will be run for each of those results. So, the expression
     * .[] | .foo retrieves the "foo" field of each element of the input array. Note
     * that .a.b.c is the same as .a | .b | .c. Note too that . is the input value at
     * the particular stage in a "pipeline", specifically: where the . expression
     * appears. Thus .a | . | .b is the same as .a.b, as the . in the middle refers
     * to whatever value .a produced.
     */

    // Step 1, read the input string, determine execution order
    // Step 2: access the toml_file to get strings, tables, etc
    // Step 3: handle various piping scenarios
    // Step 4: output
    let filter_str = matches.value_of("filter").unwrap();
    // break out string into various arrays so that we can pipe each value
    // into the next filter.
    let _filters = filter_str.split("|");

    println!("Reading toml file: \n\n{}", toml_file);
    let package = toml_file["package"].as_table().expect("whatever");
    for (key, val) in package {
        println!("key: {}, value: {}", key, val);
    }

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
