use lazy_static::lazy_static;
use std::{
    env::home_dir,
    process::exit,
    time::{Duration, SystemTime},
};
use xshell::{Shell, cmd};

use crate::{base, delegations::git};

const REPO: &str = "stoic";
const GIT: &str = git::GIT;
const REMOTE: &str = "https://github.com";

lazy_static! {
    static ref TWO_HOURS: Duration = Duration::from_secs(3600 * 2);
}

pub fn new_dir_and_pull(owner: String) -> Shell {
    let Some(home_dir) = home_dir() else {
        //tried to reach home dir
        eprintln!("tried to get into home dir");
        exit(1);
    };

    let home_dir = home_dir.to_str().unwrap();

    let home_sh = base::shell::new();
    home_sh.change_dir(home_dir);

    // check for repo
    if cmd!(home_sh, "cd {REPO}").run().is_err() {
        // clone inside home if error
        if let Err(err) = cmd!(home_sh, "{GIT} clone {REMOTE}/{owner}/{REPO}").run() {
            eprintln!("tried to clone {REPO}, maybe it does not exist?");
            eprintln!("{err}");
            exit(1);
        };
    };

    // we can now always change into REPO dir
    home_sh.change_dir(REPO);
    cmd!(home_sh, "pwd").run().unwrap();

    // always pull, if a change happend it will always be in a different file if done by stoic
    if cmd!(home_sh, "{GIT} pull").run().is_err() {
        cmd!(home_sh, "touch README.md").run().unwrap();
        cmd!(home_sh, "{GIT} add .").run().unwrap();
        cmd!(home_sh, "{GIT} commit -m initial").run().unwrap();
        cmd!(home_sh, "{GIT} push").run().unwrap();
    };

    home_sh
}

pub fn clean_commit_tree(shell: &Shell, commit_message: Option<String>) {
    let status = cmd!(shell, "{GIT} status").read().unwrap();
    if status.contains("nothing to commit, working tree clean") {
        return;
    }

    let msg = commit_message.unwrap_or("some_left_over_change".to_string());
    cmd!(shell, "{GIT} add .").run().unwrap();
    cmd!(shell, "{GIT} commit -m {msg}").run().unwrap();
    cmd!(shell, "{GIT} push").run().unwrap();
}

pub fn note(owner: String, editor_command: String) {
    let stoic_shell = new_dir_and_pull(owner);
    clean_commit_tree(&stoic_shell, None);

    let file_name = file_name();

    if let Err(err) = cmd!(stoic_shell, "touch {file_name}").run() {
        eprintln!("Could not create {file_name}");
        eprintln!("{err}");
        exit(1);
    }
    let current = stoic_shell.current_dir();
    let current = current.to_str().unwrap();
    println!("command {editor_command} {current}/{file_name}");
    cmd!(
        stoic_shell,
        "ghostty -e {editor_command} {current}/{file_name}"
    )
    .quiet()
    .ignore_stderr()
    .ignore_stdout()
    .run()
    .unwrap();

    clean_commit_tree(&stoic_shell, Some(file_name));
}

fn file_name() -> String {
    let date = date();
    format!("{}_{}.md", date.day, date.hour)
}

#[derive(Debug)]
struct StoicDate {
    day: String,
    hour: String,
}

fn date() -> StoicDate {
    let now = SystemTime::now();
    let now = now.checked_add(*TWO_HOURS).unwrap();

    let readable = humantime::format_rfc3339(now);

    let x = readable.to_string();
    let split = x.split("T").collect::<Vec<&str>>();
    let day = split[0];
    let hour = split[1];

    StoicDate {
        day: day.to_string(),
        hour: hour.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::stoic::date;

    #[test]
    pub fn should_create_readable_time() {
        let _ = date();
        println!("Should be able to create date")
    }
}
