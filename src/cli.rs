use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli{

    #[arg(long)]
    pub working_dir: Option<PathBuf>,

    #[arg(long)]
    colors: bool,
}