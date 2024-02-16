

use clap::Parser;
use shell::Shell;

mod info;
mod shell;
mod system;
mod cli;
mod utils;

fn main() {
    let args = cli::Cli::parse();
    if let Some(current_dir) = args.working_dir{
        println!("{}", current_dir.display());
    }

    let current_dir = system::get_default_dir();


    let mut main_shell = Shell::new(current_dir);

    main_shell.shell_loop();

}
