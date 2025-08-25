pub mod shell {
    use xshell::Shell;

    pub type SH = xshell::Shell;

    pub fn new() -> SH {
        Shell::new().expect("should be able to create a shell")
    }
}
