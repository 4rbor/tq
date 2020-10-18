#[test]
fn load_toml_from_file_without_crash() {
    let _foo = tq::load_toml::load_toml_from_file("../fixtures/test_01.toml");
}
