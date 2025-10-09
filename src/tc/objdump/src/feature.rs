use object::File;
use object::*;
use capstone::prelude::*;

// 反汇编等功能
//pub fn dump_disassemble(_obj: &File<'_>, _all: bool, _sym: Option<&str>, _source: bool, _source_comment: Option<&str>) {
//    println!("(暂未实现反汇编功能)");
//}

// 其它高级或扩展功能也放这里

pub fn dump_disassemble(obj: &File<'_>, all: bool, sym: Option<&str>, source: bool, source_comment: Option<&str>) {
    // 初始化 Capstone 反汇编器
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .build()
        .expect("无法初始化 Capstone 反汇编器");

    for section in obj.sections() {
        let name = section.name().unwrap_or("未知");
        println!("反汇编段: {}", name);

        if let Ok(data) = section.data() {
            match cs.disasm_all(data, 0x1000) {
                Ok(insns) => {
                    for insn in insns.iter() {
                        println!("0x{:x}:	{}	{}", insn.address(), insn.mnemonic().unwrap_or(""), insn.op_str().unwrap_or(""));
                    }
                }
                Err(err) => {
                    println!("反汇编失败: {}", err);
                }
            }
        } else {
            println!("无法读取段数据");
        }
    }
}

pub fn dump_full_contents(obj: &File<'_>) {
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

pub fn dump_decompress(_obj: &File<'_>) {
    println!("(暂未实现解压功能)");
}

pub fn dump_debugging(_obj: &File<'_>, tags: bool) {
    println!("(暂未实现调试信息显示: {})", if tags { "ctags风格" } else { "" });
}

pub fn dump_stabs(_obj: &File<'_>) {
    println!("(暂未实现STABS信息显示)");
}

pub fn dump_dwarf(_obj: &File<'_>, _opt: Option<&str>) {
    println!("(暂未实现DWARF信息显示)");
}

pub fn dump_process_links(_obj: &File<'_>) {
    println!("(暂未实现process-links功能)");
}

pub fn dump_ctf(_obj: &File<'_>, _sect: Option<&str>) {
    println!("(暂未实现CTF信息显示)");
}

pub fn dump_sframe(_obj: &File<'_>, _sect: Option<&str>) {
    println!("(暂未实现SFrame信息显示)");
}