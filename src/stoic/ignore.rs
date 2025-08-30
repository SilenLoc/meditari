use std::fs::OpenOptions;
use std::io::Write;

use xshell::Shell;

pub fn ignore_pattern(shell: Shell, patters: [&str; 1]) {
    let file_exists = shell.path_exists(".gitignore");
    if !file_exists {
        let current_dir = shell.current_dir();
        let target = current_dir.join(".gitignore");
        for pattern in patters {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&target)
                .map_err(|e| format!("Failed to create or open .gitignore: {e}"))
                .unwrap();
            writeln!(file, "{pattern}").unwrap();
        }
    } else {
        let file = shell.read_file(".gitignore").unwrap();
        let entries = file.lines().collect::<Vec<&str>>();
        let current_dir = shell.current_dir();
        let target = current_dir.join(".gitignore");
        for pattern in patters {
            if !entries.contains(&pattern) {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&target)
                    .map_err(|e| format!("Failed to open .gitignore: {e}"))
                    .unwrap();
                writeln!(file, "{pattern}").unwrap();
            }
        }
    }
}
