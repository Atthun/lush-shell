use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use crate::dispatcher;
use crate::prompt;

pub async fn shell() {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin).lines();

    loop {
        prompt::print_prompt().await;
        io::stdout().flush().await.unwrap();
        if let Ok(Some(line)) = reader.next_line().await {
            let trimmed = line.trim();
            if trimmed == "exit" {
                break;
            }
            let mut cmd_parts = trimmed.split_whitespace();
            if let Some(command) = cmd_parts.next() {
                let args: Vec<&str> = cmd_parts.collect();
                dispatcher::run_command(command, &args).await;
            }
        }
    }
}

