use crate::{delegations::gh, stoic::init::stoic_shell};

pub fn open_repo(owner: String) {
    let stoic_shell = stoic_shell(owner);
    gh::open_repo(stoic_shell);
}
