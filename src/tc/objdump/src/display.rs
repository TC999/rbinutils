use object::File;
use object::*;

pub fn dump_archive_headers(_obj: &File<'_>) {
    println!("(暂未实现归档头部显示)");
}

// 其它原来的显示函数都放这里
// pub fn dump_file_header(obj: &File<'_>) { ... }
// pub fn dump_section_headers(obj: &File<'_>) { ... }
// ...

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