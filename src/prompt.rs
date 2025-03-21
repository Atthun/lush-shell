use std::env;

pub async fn print_prompt() {
    let cwd = env::current_dir().unwrap();
    let cwd_str = cwd.to_str().unwrap_or("");
    eprint!("lush-shell [{}] > ", cwd_str);
}

