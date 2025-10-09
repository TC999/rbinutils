use object::File;
use object::*;
use object::read::archive::ArchiveFile;

/// 显示归档头信息
pub fn dump_archive_headers(data: &[u8]) {
    match ArchiveFile::parse(data) {
        Ok(archive) => {
            println!("归档成员数: {}", archive.members().count());
            for (i, member_result) in archive.members().enumerate() {
                match member_result {
                    Ok(member) => {
                        // name() 返回 &[u8]
                        let name_bytes = member.name();
                        // 尝试将 &[u8] 转为字符串
                        let name_str = std::str::from_utf8(name_bytes).unwrap_or("未知");
                        println!("成员 {}: 名称={}", i + 1, name_str);
                    }
                    Err(_) => {
                        println!("成员 {}: 解析失败", i + 1);
                    }
                }
            }
        }
        Err(_) => {
            println!("该文件不是归档文件或暂不支持归档格式。");
        }
    }
}

/// 显示文件头信息
pub fn dump_file_header(obj: &File<'_>) {
    println!("文件架构: {:?}", obj.architecture());
    println!("文件类型: {:?}", obj.kind());
    println!("字节序: {:?}", obj.endianness());
    println!("入口地址: 0x{:x}", obj.entry());
}

pub fn dump_private_headers(_obj: &File<'_>) {
    // TODO: 私有头部显示，object crate暂不支持
    println!("(暂未实现目标格式专用头部显示)");
}

pub fn dump_private(_obj: &File<'_>, opt: &str) {
    println!("(暂未实现 -P/--private: {})", opt);
}

pub fn dump_section_headers(obj: &File<'_>) {
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

pub fn dump_all_headers(obj: &File<'_>) {
    dump_file_header(obj);
    dump_section_headers(obj);
    // 其它头部按需增加
}

pub fn dump_symbols(obj: &File<'_>) {
    println!("暂未实现符号表显示");
    //for symbol in obj.symbols() {
    //    println!(
    //        "名称: {:<20} 类型: {:?} 地址: 0x{:08x} 大小: 0x{:x}",
    //        symbol.name().unwrap_or("未知"),
    //        symbol.kind(),
    //        symbol.address(),
    //        symbol.size()
    //    );
    //}
}