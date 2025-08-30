use std::{env::home_dir, process::exit};

use xshell::{Shell, cmd};

use crate::{
    base,
    delegations::{gh, git},
    stoic::ignore,
};

const REPO: &str = "stoic";
const GIT: &str = git::GIT;
const REMOTE: &str = gh::REMOTE;

pub fn stoic_shell(owner: String) -> Shell {
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

    ignore::ignore_pattern(home_sh.clone(), ["*_day.md"; 32]);

    home_sh
}
