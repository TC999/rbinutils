use std::{env, process::Command};

const VERSION: &str = "binutils-rs 0.1.0 (multi-call binary)";

const FUNCTIONS: &[&str] = &[
    "strings",
    "objdump",
    // 可以继续加入其他命令
];

fn print_usage() {
    println!("{VERSION}\n");
    println!("Usage: binutils [function [arguments...]]");
    println!("       binutils --list\n");
    println!("Options:");
    println!("      --list    lists all defined functions, one per row\n");
    println!("Currently defined functions:\n");
    for func in FUNCTIONS {
        println!("    {func}");
    }
}

fn print_functions() {
    for func in FUNCTIONS {
        println!("{func}");
    }
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None => print_usage(),
        Some("--list") => print_functions(),
        Some(cmd) => {
            if FUNCTIONS.contains(&cmd) {
                // 执行 src/命令名/target/release/命令名
                let status = Command::new(cmd)
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