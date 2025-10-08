mod args;
mod scan_object;
mod scan_plain;
mod help;

use args::Args;
use scan_object::print_strings_in_object_file;
use scan_plain::print_strings_in_file;
use help::HELP_LINES;

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.iter().any(|arg| arg == "-h" || arg == "--help") {
        for line in HELP_LINES {
            println!("{}", line);
        }
        return;
    }

    // 下面保持原样
    let args = Args::parse();

    if args.scan_object {
        match print_strings_in_object_file(&args.filename, args.object_min_len) {
            Ok(true) => {}
            Ok(false) | Err(_) => {
                if let Err(e) = print_strings_in_file(&args.filename, args.min_len) {
                    eprintln!("错误: {}", e);
                    std::process::exit(2);
                }
            }
        }
    } else {
        if let Err(e) = print_strings_in_file(&args.filename, args.min_len) {
            eprintln!("错误: {}", e);
            std::process::exit(2);
        }
    }
}