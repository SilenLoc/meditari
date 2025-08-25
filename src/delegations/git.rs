use std::process::exit;

use xshell::{Shell, cmd};

use crate::base;

pub const GIT: &str = "git";

pub fn get_username() -> String {
    let sh = base::shell::new();
    if let Ok(user_name) = cmd!(sh, "git config user.name").read() {
        return user_name;
    };
    eprintln!("Could not get user name with git");
    exit(1);
}

pub fn revert_last_commit(shell: &Shell) {
    cmd!(shell, "git revert HEAD --no-edit").run().unwrap();
}
