use toml;

fn main() {
    let matches = tq::read_args();

    let toml_file: toml::Value = match matches.value_of("file") {
        Some(file) => tq::load_toml::load_toml_from_file(file).unwrap(),
        None => tq::load_toml::load_toml_from_stdin().unwrap(),
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
    let full_filter_string = matches.value_of("filter").unwrap();
    let value = tq::filter::apply_filters(toml_file, full_filter_string);

    println!("{}", value);
    std::process::exit(0);
}
