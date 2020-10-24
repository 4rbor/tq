// Testing utility functions
pub fn get_fixture_path(file_name: &str) -> String {
    format!(
        "{project_root}/tests/fixtures/{file_name}",
        project_root = env!("CARGO_MANIFEST_DIR"),
        file_name = file_name
    )
}
