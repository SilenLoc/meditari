use std::process::exit;
use xshell::cmd;

mod date;
use date::{date, file_name};
mod revert;
pub use revert::revert;
mod reset;
pub use reset::reset;
mod commit;
use commit::commit_all;
mod init;
use init::stoic_shell;

pub fn note(owner: String, editor_command: String, content: Option<String>) {
    let stoic_shell = stoic_shell(owner);

    commit_all(&stoic_shell, None);

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
            let preview = preview(content.clone());
            println!("{preview}");
            commit_all(&stoic_shell, Some(file_name));
        } else {
            println!("No content in the file, nothing to commit");
            if let Err(err) = stoic_shell.remove_path(&file_name) {
                eprintln!("Could not delete file {err}");
                exit(1);
            };
        }
    };
}

fn preview(content: impl Into<String>) -> String {
    let content: String = content.into();
    let mut preview = content.clone();
    preview.truncate(30);
    let line_count = content.lines().count();
    let preview_line_count = preview.lines().count();
    let rest_line_count = line_count - preview_line_count;
    format!(
        "Commiting {preview}...
[{rest_line_count} more Lines]"
    )
}
