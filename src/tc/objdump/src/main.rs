use clap::{Arg, Command};
use object::{Object, ObjectSection, ObjectSymbol, File};
use std::fs;
use std::path::Path;
use std::io::{self, Write};

fn dump_archive_headers(_obj: &File<'_>) {
    // TODO: 归档头部显示，object crate暂不支持
    println!("(暂未实现归档头部显示)");
}

fn dump_file_header(obj: &File<'_>) {
    println!("文件架构: {:?}", obj.architecture());
    println!("文件类型: {:?}", obj.kind());
    println!("字节序: {:?}", obj.endianness());
    println!("入口地址: 0x{:x}", obj.entry());
}

fn dump_private_headers(_obj: &File<'_>) {
    // TODO: 私有头部显示，object crate暂不支持
    println!("(暂未实现目标格式专用头部显示)");
}

fn dump_private(_obj: &File<'_>, opt: &str) {
    println!("(暂未实现 -P/--private: {})", opt);
}

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

fn dump_all_headers(obj: &File<'_>) {
    dump_file_header(obj);
    dump_section_headers(obj);
    // 其它头部按需增加
}

fn dump_disassemble(_obj: &File<'_>, _all: bool, _sym: Option<&str>, _source: bool, _source_comment: Option<&str>) {
    // TODO: 用capstone/iced-x86等库实现反汇编
    println!("(暂未实现反汇编功能)");
}

fn dump_full_contents(obj: &File<'_>) {
    for section in obj.sections() {
        let name = section.name().unwrap_or("未知");
        println!("段 {} 内容 (前128字节):", name);
        let data = section.data().unwrap_or(&[]);
        for (i, byte) in data.iter().take(128).enumerate() {
            print!("{:02x} ", byte);
            if (i+1) % 16 == 0 { println!(); }
        }
        println!("\n");
    }
}

fn dump_decompress(_obj: &File<'_>) {
    println!("(暂未实现解压功能)");
}

fn dump_debugging(_obj: &File<'_>, tags: bool) {
    println!("(暂未实现调试信息显示: {})", if tags { "ctags风格" } else { "" });
}

fn dump_stabs(_obj: &File<'_>) {
    println!("(暂未实现STABS信息显示)");
}

fn dump_dwarf(_obj: &File<'_>, _opt: Option<&str>) {
    println!("(暂未实现DWARF信息显示)");
}

fn dump_process_links(_obj: &File<'_>) {
    println!("(暂未实现process-links功能)");
}

fn dump_ctf(_obj: &File<'_>, _sect: Option<&str>) {
    println!("(暂未实现CTF信息显示)");
}

fn dump_sframe(_obj: &File<'_>, _sect: Option<&str>) {
    println!("(暂未实现SFrame信息显示)");
}

fn dump_symbols(obj: &File<'_>, dynamic: bool) {
    if dynamic {
        println!("动态符号表:");
        for sym in obj.dynamic_symbols() {
            println!(
                "名称: {:<30} 地址: 0x{:08x} 大小: 0x{:x} 类型: {:?}",
                sym.name().unwrap_or("未知"),
                sym.address(),
                sym.size(),
                sym.kind()
            );
        }
    } else {
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
}

fn dump_reloc(_obj: &File<'_>, dynamic: bool) {
    println!("(暂未实现{}重定位信息显示)", if dynamic { "动态" } else { "" });
}

fn dump_version() {
    println!("rust-objdump 0.1.0");
}

fn dump_info() {
    println!("支持的格式: ELF, COFF, Mach-O 等 (object crate)");
    println!("支持的架构: x86, x86_64, ARM, AArch64, MIPS, PowerPC 等");
}

fn print_usage() {
    println!("Usage: rust-objdump <option(s)> <file(s)>");
    println!("显示目标文件信息。至少需要指定一个选项：");
    println!("  -a, --archive-headers    显示归档头信息");
    println!("  -f, --file-headers       显示文件头信息");
    println!("  -p, --private-headers    显示目标格式专用头部");
    println!("  -P, --private=OPT,OPT... 显示目标格式专用内容");
    println!("  -h, --section-headers    显示段头信息");
    println!("  -x, --all-headers        显示所有头部");
    println!("  -d, --disassemble        反汇编可执行段内容");
    println!("  -D, --disassemble-all    反汇编所有段内容");
    println!("      --disassemble=<sym>  仅反汇编指定符号");
    println!("  -S, --source             反汇编中夹带源代码");
    println!("      --source-comment[=txt] 源代码行前缀");
    println!("  -s, --full-contents      显示所有段内容");
    println!("  -Z, --decompress         解压后显示段内容");
    println!("  -g, --debugging          显示调试信息");
    println!("  -e, --debugging-tags     用ctags风格显示调试信息");
    println!("  -G, --stabs              显示STABS信息");
    println!("  -W, --dwarf[=opt]        显示DWARF调试信息");
    println!("  -L, --process-links      显示分离调试文件内容");
    println!("      --ctf[=SECTION]      显示CTF信息");
    println!("      --sframe[=SECTION]   显示SFrame信息");
    println!("  -t, --syms               显示符号表");
    println!("  -T, --dynamic-syms       显示动态符号表");
    println!("  -r, --reloc              显示重定位信息");
    println!("  -R, --dynamic-reloc      显示动态重定位信息");
    println!("  -v, --version            显示版本号");
    println!("  -i, --info               显示支持的格式和架构");
    println!("  -H, --help               显示此帮助信息");
}

fn main() -> io::Result<()> {
    let matches = Command::new("rust-objdump")
        .version("0.1.0")
        .about("Rust实现的objdump命令")
        .disable_help_flag(false) // 保留自动帮助 -h
        .arg(Arg::new("archive-headers")
            .short('a')
            .long("archive-headers")
            .help("显示归档头信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("file-headers")
            .short('f')
            .long("file-headers")
            .help("显示文件头信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("private-headers")
            .short('p')
            .long("private-headers")
            .help("显示目标格式专用头部")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("private")
            .short('P')
            .long("private")
            .help("显示目标格式专用内容")
            .num_args(1))
        .arg(Arg::new("section-headers")
            .long("section-headers")
            .help("显示段头信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("all-headers")
            .short('x')
            .long("all-headers")
            .help("显示所有头部")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("disassemble")
            .short('d')
            .long("disassemble")
            .help("反汇编可执行段内容")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("disassemble-all")
            .short('D')
            .long("disassemble-all")
            .help("反汇编所有段内容")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("disassemble-sym")
            .long("disassemble")
            .help("仅反汇编指定符号")
            .num_args(1))
        .arg(Arg::new("source")
            .short('S')
            .long("source")
            .help("反汇编中夹带源代码")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("source-comment")
            .long("source-comment")
            .help("源代码行前缀")
            .num_args(1))
        .arg(Arg::new("full-contents")
            .short('s')
            .long("full-contents")
            .help("显示所有段内容")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("decompress")
            .short('Z')
            .long("decompress")
            .help("解压后显示段内容")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("debugging")
            .short('g')
            .long("debugging")
            .help("显示调试信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("debugging-tags")
            .short('e')
            .long("debugging-tags")
            .help("用ctags风格显示调试信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("stabs")
            .short('G')
            .long("stabs")
            .help("显示STABS信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("dwarf")
            .short('W')
            .long("dwarf")
            .help("显示DWARF信息")
            .num_args(1))
        .arg(Arg::new("process-links")
            .short('L')
            .long("process-links")
            .help("显示分离调试文件内容")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("ctf")
            .long("ctf")
            .help("显示CTF信息")
            .num_args(1))
        .arg(Arg::new("sframe")
            .long("sframe")
            .help("显示SFrame信息")
            .num_args(1))
        .arg(Arg::new("syms")
            .short('t')
            .long("syms")
            .help("显示符号表")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("dynamic-syms")
            .short('T')
            .long("dynamic-syms")
            .help("显示动态符号表")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("reloc")
            .short('r')
            .long("reloc")
            .help("显示重定位信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("dynamic-reloc")
            .short('R')
            .long("dynamic-reloc")
            .help("显示动态重定位信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("version")
            .short('v')
            .long("version")
            .help("显示版本号")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("info")
            .short('i')
            .long("info")
            .help("显示支持的格式和架构")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("help")
            .short('H')
            .long("help")
            .help("显示帮助信息")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("input")
            .help("目标文件路径")
            .required(true)
            .index(1))
        .get_matches();

    if matches.get_flag("help") {
        print_usage();
        return Ok(());
    }
    if matches.get_flag("version") {
        dump_version();
        return Ok(());
    }
    if matches.get_flag("info") {
        dump_info();
        return Ok(());
    }

    let input = matches.get_one::<String>("input").unwrap();
    let data = fs::read(Path::new(input))?;
    let obj = File::parse(&*data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("解析目标文件失败: {}", e)))?;

    if matches.get_flag("archive-headers") { dump_archive_headers(&obj); }
    if matches.get_flag("file-headers") { dump_file_header(&obj); }
    if matches.get_flag("private-headers") { dump_private_headers(&obj); }
    if let Some(opt) = matches.get_one::<String>("private") { dump_private(&obj, opt); }
    if matches.get_flag("section-headers") { dump_section_headers(&obj); }
    if matches.get_flag("all-headers") { dump_all_headers(&obj); }
    if matches.get_flag("disassemble") { dump_disassemble(&obj, false, None, false, None); }
    if matches.get_flag("disassemble-all") { dump_disassemble(&obj, true, None, false, None); }
    if let Some(sym) = matches.get_one::<String>("disassemble-sym") { dump_disassemble(&obj, false, Some(sym), false, None); }
    let source = matches.get_flag("source");
    let source_comment = matches.get_one::<String>("source-comment");
    if source || source_comment.is_some() { dump_disassemble(&obj, false, None, source, source_comment.map(|s| s.as_str())); }
    if matches.get_flag("full-contents") { dump_full_contents(&obj); }
    if matches.get_flag("decompress") { dump_decompress(&obj); }
    if matches.get_flag("debugging") { dump_debugging(&obj, false); }
    if matches.get_flag("debugging-tags") { dump_debugging(&obj, true); }
    if matches.get_flag("stabs") { dump_stabs(&obj); }
    if let Some(opt) = matches.get_one::<String>("dwarf") { dump_dwarf(&obj, Some(opt)); }
    if matches.get_flag("process-links") { dump_process_links(&obj); }
    if let Some(sect) = matches.get_one::<String>("ctf") { dump_ctf(&obj, Some(sect)); }
    if let Some(sect) = matches.get_one::<String>("sframe") { dump_sframe(&obj, Some(sect)); }
    if matches.get_flag("syms") { dump_symbols(&obj, false); }
    if matches.get_flag("dynamic-syms") { dump_symbols(&obj, true); }
    if matches.get_flag("reloc") { dump_reloc(&obj, false); }
    if matches.get_flag("dynamic-reloc") { dump_reloc(&obj, true); }

    // 如果未指定任何选项，显示帮助
    let no_flag = !matches.get_flag("archive-headers")
        && !matches.get_flag("file-headers")
        && !matches.get_flag("private-headers")
        && matches.get_one::<String>("private").is_none()
        && !matches.get_flag("section-headers")
        && !matches.get_flag("all-headers")
        && !matches.get_flag("disassemble")
        && !matches.get_flag("disassemble-all")
        && matches.get_one::<String>("disassemble-sym").is_none()
        && !source
        && source_comment.is_none()
        && !matches.get_flag("full-contents")
        && !matches.get_flag("decompress")
        && !matches.get_flag("debugging")
        && !matches.get_flag("debugging-tags")
        && !matches.get_flag("stabs")
        && matches.get_one::<String>("dwarf").is_none()
        && !matches.get_flag("process-links")
        && matches.get_one::<String>("ctf").is_none()
        && matches.get_one::<String>("sframe").is_none()
        && !matches.get_flag("syms")
        && !matches.get_flag("dynamic-syms")
        && !matches.get_flag("reloc")
        && !matches.get_flag("dynamic-reloc");
    if no_flag {
        print_usage();
    }
    Ok(())
}