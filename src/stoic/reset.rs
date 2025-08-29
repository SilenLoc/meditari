use std::process::exit;

use xshell::cmd;

use crate::{
    delegations::git,
    stoic::{commit_all, stoic_shell},
};
const GIT: &str = git::GIT;

pub fn reset(owner: String) {
    let shell = stoic_shell(owner);
    if cmd!(shell, "{GIT} rm -r *.md").quiet().run().is_err() {
        eprintln!("no markdown files found to delete");
        exit(1)
    };
    commit_all(&shell, Some("reset all".to_string()));
    println!("you may revert this by running revert")
}
