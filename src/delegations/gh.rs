use xshell::{Shell, cmd};

pub const GH: &str = "gh";
pub const REMOTE: &str = "https://github.com";

pub fn open_repo(shell: Shell) {
    cmd!(shell, "{GH} browse")
        .quiet()
        .ignore_stderr()
        .run()
        .unwrap();
}
