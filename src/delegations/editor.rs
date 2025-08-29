use std::env;

pub fn find_editor() -> String {
    env::var("EDITOR").unwrap_or("zed".to_string())
}
