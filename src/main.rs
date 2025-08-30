use clap::{Parser, Subcommand, command};

use crate::delegations::{editor, git};

mod base;
mod delegations;
mod stoic;

#[derive(Parser)]
#[command(name = "med")]
#[command(about = "")]
struct Cli {
    /// Optional name to operate on
    owner: Option<String>,
    editor: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Reset {},
    Test {},
    Revert {},
    Browse {},
    RemoveEmpty {},
    Content {
        #[arg()]
        content: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let owner = cli.owner.unwrap_or(git::get_username());
    let editor_command = cli.editor.unwrap_or(editor::find_editor());

    match &cli.command {
        Some(Commands::Test {}) => {
            // only to see if the cli actually runs
        }
        Some(Commands::Reset {}) => {
            stoic::reset(owner);
        }
        Some(Commands::Revert {}) => {
            stoic::revert(owner);
        }
        Some(Commands::RemoveEmpty {}) => {
            stoic::remove_empty_files(owner);
        }
        Some(Commands::Content { content }) => {
            stoic::note(owner, editor_command, Some(content.to_string()));
        }
        Some(Commands::Browse {}) => {
            stoic::open_repo(owner);
        }
        None => {
            stoic::note(owner, editor_command, None);
        }
    }
}
