pub fn apply_filters(toml_file: toml::Value, filter: &str) -> toml::Value {
    // filter.split("|").fold(&toml_file, |acc, filter_str| {
    //     // Identity filter
    //     if filter_str.trim() == "." {
    //         return acc;
    //     }

    //     let keys = filter_str.split(".");
    //     let _count = filter_str.split(".").count();

    //     let mut toml_value = &toml_file;
    //     for key in keys {
    //         let trimmed_key = key.trim();
    //         if trimmed_key == "" {
    //             continue;
    //         }

    //         toml_value = toml_value.get(key).unwrap();
    //     }
    //     return toml_value;
    // })
    return toml_file;
}
