use std::{io, fs::File, io::{Read, BufReader}, path::Path};

pub fn print_strings_in_file<P: AsRef<Path>>(path: P, min_len: usize) -> io::Result<()> {
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