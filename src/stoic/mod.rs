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

pub fn reset(owner: String) {
    let shell = new_dir_and_pull(owner);
    if cmd!(shell, "{GIT} rm -r *.md").quiet().run().is_err() {
        eprintln!("no markdown files found to delete");
        exit(1)
    };
    clean_commit_tree(&shell, Some("reset all".to_string()));
    println!("you may revert this by running revert")
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
    if cmd!(home_sh, "cd {REPO}").quiet().run().is_err() {
        // clone inside home if error
        if let Err(err) = cmd!(home_sh, "{GIT} clone {REMOTE}/{owner}/{REPO}")
            .quiet()
            .ignore_stdout()
            .run()
        {
            eprintln!("tried to clone {REPO}, maybe it does not exist?");
            eprintln!("{err}");
            exit(1);
        };
    };

    // we can now always change into REPO dir
    home_sh.change_dir(REPO);

    cmd!(home_sh, "{GIT} checkout main -q")
        .ignore_stdout()
        .quiet()
        .run()
        .unwrap();

    // always pull, if a change happend it will always be in a different file if done by stoic
    if cmd!(home_sh, "{GIT} pull -q")
        .ignore_stdout()
        .quiet()
        .run()
        .is_err()
    {
        cmd!(home_sh, "touch README.md")
            .ignore_stdout()
            .quiet()
            .run()
            .unwrap();
        cmd!(home_sh, "{GIT} add . -q")
            .ignore_stdout()
            .quiet()
            .run()
            .unwrap();
        cmd!(home_sh, "{GIT} commit -m initial")
            .quiet()
            .ignore_stdout()
            .run()
            .unwrap();
        cmd!(home_sh, "{GIT} push")
            .quiet()
            .ignore_stdout()
            .run()
            .unwrap();
    };

    home_sh
}

pub fn clean_commit_tree(shell: &Shell, commit_message: Option<String>) {
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

pub fn revert(owner: String) {
    let stoic_shell = new_dir_and_pull(owner);
    git::revert_last_commit(&stoic_shell);
    clean_commit_tree(&stoic_shell, Some("revert".to_string()));
}

pub fn note(owner: String, editor_command: String, content: Option<String>) {
    let stoic_shell = new_dir_and_pull(owner);

    clean_commit_tree(&stoic_shell, None);

    let date = date();
    let file_name = file_name(&date);

    let dir_name = format!("{}/{}/{}", date.year(), date.month(), date.day());

    if !stoic_shell.path_exists(&dir_name) {
        if let Err(err) = stoic_shell.create_dir(&dir_name) {
            eprintln!("{err}");
            exit(1);
        };
    }

    stoic_shell.change_dir(&dir_name);

    if let Err(err) = cmd!(stoic_shell, "touch {file_name}")
        .quiet()
        .ignore_stdout()
        .run()
    {
        eprintln!("Could not create {file_name}");
        eprintln!("{err}");
        exit(1);
    }
    let current = stoic_shell.current_dir();
    let current = current.to_str().unwrap();

    let full_file_path = format!("{current}/{file_name}");

    if let Some(content) = content {
        stoic_shell
            .write_file(&file_name, content.as_bytes())
            .unwrap();
    } else {
        cmd!(stoic_shell, "ghostty -e {editor_command} {full_file_path}")
            .quiet()
            .ignore_stderr()
            .ignore_stdout()
            .run()
            .unwrap();
    }

    if let Ok(content) = stoic_shell.read_file(&file_name) {
        if !content.is_empty() {
            let mut preview = content.clone();
            preview.truncate(30);
            let line_count = content.lines().count();
            let preview_line_count = preview.lines().count();
            let rest_line_count = line_count - preview_line_count;
            println!(
                "Commiting {preview}...
[{rest_line_count} more Lines]"
            );
            clean_commit_tree(&stoic_shell, Some(file_name));
        } else {
            println!("No content in the file, nothing to commit");
            if let Err(err) = stoic_shell.remove_path(&file_name) {
                eprintln!("Could not delete file {err}");
                exit(1);
            };
        }
    };
}

fn file_name(date: &StoicDate) -> String {
    format!("{}_{}_{}_{}.md", date.year, date.month, date.day, date.rest)
}

#[derive(Debug, Clone)]
struct StoicDate {
    year: String,
    month: String,
    day: String,
    rest: String,
}

impl StoicDate {
    fn year(&self) -> String {
        self.year.clone()
    }

    fn month(&self) -> String {
        self.month.clone()
    }

    fn day(&self) -> String {
        self.day.clone()
    }
}

fn date() -> StoicDate {
    let now = SystemTime::now();
    let now = now.checked_add(*TWO_HOURS).unwrap();

    let readable = humantime::format_rfc3339(now);

    let x = readable.to_string();
    let split = x.split("T").collect::<Vec<&str>>();
    let rest = split[1].replace(".", "_");

    let year_month_day = split[0];
    let split = year_month_day.split("-").collect::<Vec<&str>>();
    let year = split[0];
    let month = split[1];
    let day = split[2];

    StoicDate {
        day: day.to_string(),
        month: month.to_string(),
        year: year.to_string(),
        rest: rest.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::stoic::date;

    #[test]
    pub fn should_create_readable_time() {
        let _date = date();
        println!("Should be able to create date")
    }
}
