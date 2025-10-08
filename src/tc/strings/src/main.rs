//use std::{env, fs::File, io::{self, Read, BufReader}, path::Path};
//use object::{Object, ObjectSection};
use std::{env, fs::File, io::{self, Read, BufReader}, path::Path};
use object::{Object, ObjectSection, SectionKind};

/// 扫描对象文件的数据段，打印可打印字符串
fn print_strings_in_object_file<P: AsRef<Path>>(path: P, min_len: usize) -> io::Result<bool> {
    let data = std::fs::read(&path)?;
    let obj_file = match object::File::parse(&*data) {
        Ok(f) => f,
        Err(_) => return Ok(false), // 不是对象文件
    };

    let mut found = false;
    for section in obj_file.sections() {
        // 判断是不是已初始化数据段
        if section.kind() == SectionKind::Data || section.kind() == SectionKind::ReadOnlyData {
            let buf = section.data().unwrap_or(&[]);
            if buf.is_empty() {
                continue;
            }
            let mut current = Vec::new();
            for &byte in buf {
                if byte.is_ascii_graphic() || byte == b' ' || byte == b'\t' {
                    current.push(byte);
                } else {
                    if current.len() >= min_len {
                        let s = String::from_utf8_lossy(&current);
                        println!("{}:{} {}", path.as_ref().display(), section.name().unwrap_or(""), s);
                        found = true;
                    }
                    current.clear();
                }
            }
            if current.len() >= min_len {
                let s = String::from_utf8_lossy(&current);
                println!("{}:{} {}", path.as_ref().display(), section.name().unwrap_or(""), s);
                found = true;
            }
        }
    }
    Ok(found)
}

/// 普通文件扫描
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
    if current.len() >= min_len {
        let s = String::from_utf8_lossy(&current);
        println!("{}", s);
    }
    Ok(())
}

fn main() {
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

    // 优先尝试对象文件数据段扫描，失败则退回整个文件扫描
    match print_strings_in_object_file(file_arg, min_len) {
        Ok(true) => {}
        Ok(false) | Err(_) => {
            let _ = print_strings_in_file(file_arg, min_len);
        }
    }
}