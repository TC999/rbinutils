use clap::{Arg, Command};
use object::{Object, ObjectSection, ObjectSymbol, File};
use std::fs;
use std::path::Path;
use std::io::{self};

/// 显示文件头信息
fn dump_file_header(obj: &File<'_>) {
    println!("文件架构: {:?}", obj.architecture());
    println!("文件类型: {:?}", obj.kind());
    println!("字节序: {:?}", obj.endianness());
    println!("入口地址: 0x{:x}", obj.entry());
}

/// 显示段信息
fn dump_section_headers(obj: &File<'_>) {
    println!("段信息:");
    for section in obj.sections() {
        println!(
            "名称: {:<20} 类型: {:?} 地址: 0x{:08x} 大小: 0x{:x}",
            section.name().unwrap_or("未知"),
            section.kind(),
            section.address(),
            section.size()
        );
    }
}

/// 显示符号表
fn dump_symbols(obj: &File<'_>) {
    println!("符号表:");
    for sym in obj.symbols() {
        println!(
            "名称: {:<30} 地址: 0x{:08x} 大小: 0x{:x} 类型: {:?}",
            sym.name().unwrap_or("未知"),
            sym.address(),
            sym.size(),
            sym.kind()
        );
    }
}

/// 显示所有段内容（十六进制）
fn dump_section_contents(obj: &File<'_>) {
    for section in obj.sections() {
        let name = section.name().unwrap_or("未知");
        println!("段 {} 内容 (前 128字节):", name);
        let data = section.data().unwrap_or(&[]);
        for (i, byte) in data.iter().take(128).enumerate() {
            print!("{:02x} ", byte);
            if (i+1) % 16 == 0 { println!(); }
        }
        println!("\n");
    }
}

// TODO: 你可以用 object::read::elf::File/COFFFile 等专用类型实现更详细的功能
// TODO: 反汇编功能可以用 capstone、iced-x86 或 llvm-sys 封装实现

fn main() -> io::Result<()> {
    let matches = Command::new("rust-objdump")
        .version("0.1.0")
        .author("你的名字")
        .about("Rust 实现的 objdump")
        .arg(Arg::new("input")
            .help("目标文件路径")
            .required(true)
            .index(1))
        .arg(Arg::new("file-header")
            .short('f')
            .help("显示文件头"))
        .arg(Arg::new("section-headers")
            .short('h')
            .help("显示段头"))
        .arg(Arg::new("symbols")
            .short('t')
            .help("显示符号表"))
        .arg(Arg::new("full-contents")
            .short('s')
            .help("显示所有段内容"))
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    let data = fs::read(Path::new(input))?;
    let obj = File::parse(&*data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("解析目标文件失败: {}", e)))?;

    if matches.get_flag("file-header") {
        dump_file_header(&obj);
    }
    if matches.get_flag("section-headers") {
        dump_section_headers(&obj);
    }
    if matches.get_flag("symbols") {
        dump_symbols(&obj);
    }
    if matches.get_flag("full-contents") {
        dump_section_contents(&obj);
    }
    // 默认行为：如未指定参数则全部显示
    if !matches.get_flag("file-header")
        && !matches.get_flag("section-headers")
        && !matches.get_flag("symbols")
        && !matches.get_flag("full-contents")
    {
        dump_file_header(&obj);
        dump_section_headers(&obj);
        dump_symbols(&obj);
        dump_section_contents(&obj);
    }
    Ok(())
}