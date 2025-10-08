use std::path::Path;
use object::{Object, ObjectSection, SectionKind};
use std::io;

pub fn print_strings_in_object_file<P: AsRef<Path>>(path: P, min_len: usize) -> io::Result<bool> {
    let data = std::fs::read(&path)?;
    let obj_file = match object::File::parse(&*data) {
        Ok(f) => f,
        Err(_) => return Ok(false),
    };

    let mut found = false;
    for section in obj_file.sections() {
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
                        // 只输出字符串内容，不输出路径和段名
                        println!("{}", s);
                        found = true;
                    }
                    current.clear();
                }
            }
            if current.len() >= min_len {
                let s = String::from_utf8_lossy(&current);
                // 只输出字符串内容，不输出路径和段名
                println!("{}", s);
                found = true;
            }
        }
    }
    Ok(found)
}