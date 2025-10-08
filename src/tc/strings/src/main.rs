use std::{env, fs::File, io::{self, Read, BufReader}, path::Path};

/// 打印文件中的可打印字符串
fn print_strings_in_file<P: AsRef<Path>>(path: P, min_len: usize) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut current = Vec::new();
    for &byte in &buffer {
        if byte.is_ascii_graphic() || byte == b' ' || byte == b'\t' {
            current.push(byte);
        } else {
            if current.len() >= min_len {
                let s = String::from_utf8_lossy(&current);
                println!("{}", s);
            }
            current.clear();
        }
    }
    // 结尾可能还有剩余
    if current.len() >= min_len {
        let s = String::from_utf8_lossy(&current);
        println!("{}", s);
    }
    Ok(())
}

fn main() {
    // 默认最小字符串长度为4
    let mut min_len = 4usize;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: strings <filename> [-n min_len]");
        std::process::exit(1);
    }
    let mut file_arg = "";
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-n" => {
                if i + 1 < args.len() {
                    min_len = args[i + 1].parse().unwrap_or(4);
                    i += 1;
                }
            }
            filename => {
                file_arg = filename;
            }
        }
        i += 1;
    }

    if let Err(e) = print_strings_in_file(file_arg, min_len) {
        eprintln!("错误: {}", e);
        std::process::exit(2);
    }
}
