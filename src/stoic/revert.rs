use crate::{
    delegations::git,
    stoic::{commit_all, stoic_shell},
};

pub fn revert(owner: String) {
    let stoic_shell = stoic_shell(owner);
    git::revert_last_commit(&stoic_shell);
    commit_all(&stoic_shell, Some("revert".to_string()));
}
