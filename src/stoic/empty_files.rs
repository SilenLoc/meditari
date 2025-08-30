use std::process::exit;

use xshell::cmd;

use crate::stoic::{commit_all, stoic_shell};

pub fn remove_empty_files(owner: String) {
    let shell = stoic_shell(owner);

    // Find and remove all empty files
    if let Err(err) = cmd!(shell, "find . -type f -empty -delete").quiet().run() {
        eprintln!("Failed to remove empty files: {err}");
        exit(1);
    }

    println!("Removed all empty files");
    commit_all(&shell, Some("remove empty files".to_string()));
}
