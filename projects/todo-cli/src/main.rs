use std::io::{self, BufRead, Write};

use clap::Parser;
use todo_cli::cli::{CLI_NAME, Cli, Commands};
use todo_cli::task::{TaskManager, TaskStatus};

fn main() {
    println!("TODO-CLI Starting...");

    let cli_name = CLI_NAME;

    let mut task_manager = TaskManager::new();
    task_manager.init();

    println!("人物列表加载完成，记录数：{}", task_manager.count(None));

    let stdin = io::stdin();
    loop {
        print!("{cli_name}> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if stdin.lock().read_line(&mut input).unwrap_or(0) == 0 {
            continue;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        };

        let args = trimmed.split_whitespace();

        match Cli::try_parse_from(args) {
            Ok(cmd) => handle_command(&mut task_manager, cmd.command),
            Err(e) => {
                e.print().unwrap();
            }
        }
    }
}

fn handle_command(task_manager: &mut TaskManager, _cmd: Commands) {
    match _cmd {
        Commands::Add { content } => {
            task_manager.add(&content);
        }
        Commands::List { done, pending, .. } => {
            if done {
                task_manager.list(Some(TaskStatus::DONE));
            } else if pending {
                task_manager.list(Some(TaskStatus::PENDING));
            } else {
                task_manager.list(None);
            }
        }
        Commands::Done { id } => {
            task_manager.done(id);
        }
        Commands::Remove { id } => {
            task_manager.remove(id);
        }
        Commands::Clear => {
            task_manager.clear();
        }
        Commands::Exit => {
            std::process::exit(0);
        }
    }
}
