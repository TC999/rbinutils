mod args;
mod scan_object;
mod scan_plain;

use args::Args;
use scan_object::print_strings_in_object_file;
use scan_plain::print_strings_in_file;

fn main() {
    let args = Args::parse();

    if args.scan_object {
        // 仅当 -d 参数时，先尝试对象文件扫描
        match print_strings_in_object_file(&args.filename, args.min_len) {
            Ok(true) => {}
            Ok(false) | Err(_) => {
                if let Err(e) = print_strings_in_file(&args.filename, args.min_len) {
                    eprintln!("错误: {}", e);
                    std::process::exit(2);
                }
            }
        }
    } else {
        // 普通扫描
        if let Err(e) = print_strings_in_file(&args.filename, args.min_len) {
            eprintln!("错误: {}", e);
            std::process::exit(2);
        }
    }
}