pub fn cmd_type(args: &[&str]) {
    if args.len() != 1 {
        println!("type: expected 1 argument, got {}", args.len());
        return;
    }
    println!("{} is a shell builtin", args[0]);
}

pub fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

pub fn exit(_args: &[&str]) {
    std::process::exit(0);
}

pub const BUILD_INS: [&str; 3] = ["exit", "echo", "type"];
