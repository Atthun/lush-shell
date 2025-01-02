// Lush Shell v0.0.7

use std::{env, process::{Command, exit}, io::{self, Write}, path::{Path, PathBuf}};
use std::os::unix::fs::PermissionsExt;

fn main() {
    print!("Welcome to LushShell v0.0.7\n");  // Corrected here
    loop {
        print!("LushShell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty() { continue; }

        let mut parts = input.split_whitespace();
        if let Some(command) = parts.next() {
            if command == "exit" { exit(0); }
            if command == "type" { if let Some(arg) = parts.next() { handle_type(arg); } continue; }
            if command == "echo" { println!("{}", parts.collect::<Vec<_>>().join(" ")); continue; }

            if let Some(path) = find_command(command) {
                let output = Command::new(path).args(parts.collect::<Vec<_>>()).output();
                match output {
                    Ok(output) => {
                        if !output.stdout.is_empty() { print!("{}", String::from_utf8_lossy(&output.stdout)); }
                        if !output.stderr.is_empty() { eprint!("{}", String::from_utf8_lossy(&output.stderr)); }
                    }
                    _ => println!("{}: not found", command),
                }
            } else {
                println!("{}: not found", command);
            }
        }
    }
}

fn find_command(command: &str) -> Option<PathBuf> {
    env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .filter_map(|dir| {
            let path = Path::new(dir).join(command);
            if path.exists() && path.is_file() && is_executable(&path) {
                Some(path)
            } else {
                None
            }
        })
        .next()
}

fn is_executable(path: &Path) -> bool {
    path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

fn handle_type(arg: &str) {
    let builtins = ["echo", "exit", "type"];
    if builtins.contains(&arg) {
        println!("{} is a shell builtin", arg);
    } else if let Some(path) = find_command(arg) {
        println!("{} is {}", arg, path.display());
    } else {
        println!("{}: not found", arg);
    }
}
