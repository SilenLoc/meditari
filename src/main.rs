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
}

fn main() {
    let cli = Cli::parse();

    let owner = cli.owner.unwrap_or(git::get_username());
    let editor_command = cli.editor.unwrap_or(editor::find_editor());

    match &cli.command {
        Some(Commands::Test {}) => {
            stoic::add_test_file_and_reset(owner);
        }
        Some(Commands::Reset {}) => {
            stoic::reset(owner);
        }
        Some(Commands::Revert {}) => {
            stoic::revert(owner);
        }
        None => {
            stoic::note(owner, editor_command);
        }
    }
}
