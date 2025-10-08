use std::{env, fs, process::Command};
use std::path::PathBuf;

const VERSION: &str = "binutils-rs 0.1.0 (multi-call binary)";

fn print_usage() {
    println!("{VERSION}\n");
    println!("Usage: binutils [function [arguments...]]");
    println!("       binutils --list\n");
    println!("Options:");
    println!("      --list    lists all defined functions, one per row\n");
    println!("Currently defined functions:\n");
}

fn load_functions() -> Vec<String> {
    let mut functions = Vec::new();
    if let Ok(entries) = fs::read_dir("src/tc") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        functions.push(name.to_string());
                    }
                }
            }
        }
    }
    functions
}

fn main() {
    let functions = load_functions();
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None => print_usage(),
        Some("--list") => {
            for func in &functions {
                println!("{func}");
            }
        }
        Some(cmd) => {
            if functions.contains(&cmd.to_string()) {
                // 确保调用的是项目内的子命令
                let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target/debug".to_string());
                let cmd_path = PathBuf::from(target_dir).join(cmd);
                let status = Command::new(cmd_path)
                    .args(args)
                    .status();
                match status {
                    Ok(s) => std::process::exit(s.code().unwrap_or(1)),
                    Err(e) => {
                        eprintln!("运行 {cmd} 时出错: {e}");
                        std::process::exit(2);
                    }
                }
            } else {
                eprintln!("未知命令: {cmd}\n");
                print_usage();
                std::process::exit(127);
            }
        }
    }
}