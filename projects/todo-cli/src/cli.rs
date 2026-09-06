use clap::{Parser, Subcommand};

pub const CLI_NAME: &str = "todo-cli";

#[derive(Parser, Debug)]
#[command(name = CLI_NAME, version, about = "A simple todo CLI application", no_binary_name=true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        content: String,
    },
    List {
        #[arg(short, long, default_value_t = false)]
        all: bool,
        #[arg(short, long, default_value_t = false)]
        done: bool,
        #[arg(short, long, default_value_t = false)]
        pending: bool,
    },
    Done {
        id: u32,
    },
    Remove {
        id: u32,
    },
    Clear,
    Exit,
}
