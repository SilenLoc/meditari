use std::process::exit;

use xshell::{Shell, cmd};

use crate::delegations::git;

const GIT: &str = git::GIT;

pub fn commit_all(shell: &Shell, commit_message: Option<String>) {
    let status = cmd!(shell, "{GIT} status").quiet().read().unwrap();
    if status.contains("nothing to commit, working tree clean") {
        cmd!(shell, "{GIT} push -q")
            .quiet()
            .ignore_status()
            .ignore_stdout()
            .read()
            .unwrap();
        return;
    }

    let msg = commit_message.unwrap_or("some_left_over_change".to_string());
    cmd!(shell, "{GIT} add .")
        .quiet()
        .ignore_stdout()
        .run()
        .unwrap();
    if let Err(err) = cmd!(shell, "{GIT} commit -m {msg}")
        .quiet()
        .ignore_stdout()
        .run()
    {
        eprintln!("{err}");
        exit(1)
    }

    cmd!(shell, "{GIT} push -q")
        .quiet()
        .ignore_stdout()
        .run()
        .unwrap();
}
