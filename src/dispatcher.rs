use tokio::{process::Command};
use std::process::Stdio;
use crate::utils;

pub async fn run_command(command: &str, args: &[&str]) {
    match command {
        "cls" => {
            utils::clear_screen();
        },
        "exit" => {
            println!("Terminating lush session");
            std::process::exit(0);
        },
        _ => {
            let mut cmd = Command::new(command);
            cmd.args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());

            match cmd.spawn() {
                Ok(mut child) => {
                    let _ = child.wait().await;
                }
                Err(_) => {
                    crate::hooks::command_not_found_hook::handle_command_not_found(command);
                }
            }
        }
    }
}

