use std::env;

pub fn find_editor() -> String {
    env::var("MED_EDITOR").unwrap_or("vi".to_string())
}
