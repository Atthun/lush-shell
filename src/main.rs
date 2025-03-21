mod dispatcher;
mod prompt;
mod utils;
mod hooks;
mod shell;

#[tokio::main]
async fn main() {
    shell::shell().await;
}
