use std::{env, path::PathBuf};

use shell::Shell;

mod info;
mod shell;
mod system;
fn main() {
    let current_dir = system::get_default_dir();


    let main_shell = Shell::new(current_dir);

    main_shell.hello_command();

    info::user::get().unwrap();
}
