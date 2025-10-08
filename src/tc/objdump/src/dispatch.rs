use clap::ArgMatches;
use object::File;
use std::fs;
use std::path::Path;
use std::io;
use crate::display::*;
use crate::feature::*;

/// 分发参数并调用对应功能
pub fn dispatch(matches: &ArgMatches) -> io::Result<()> {
    //if matches.get_flag("help") {
    //    print_usage();
    //    return Ok(());
    //}
    //if matches.get_flag("version") {
    //    dump_version();
    //    return Ok(());
    //}
    //if matches.get_flag("info") {
    //    dump_info();
    //    return Ok(());
    //}

    let input = matches.get_one::<String>("input").unwrap();
    let data = fs::read(Path::new(input))?;
    let obj = File::parse(&*data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("解析目标文件失败: {}", e)))?;

    if matches.get_flag("archive-headers") { dump_archive_headers(&data); }
    if matches.get_flag("file-headers") { dump_file_header(&obj); }
    //if matches.get_flag("private-headers") { dump_private_headers(&obj); }
    //if let Some(opt) = matches.get_one::<String>("private") { dump_private(&obj, opt); }
    //if matches.get_flag("section-headers") { dump_section_headers(&obj); }
    //if matches.get_flag("all-headers") { dump_all_headers(&obj); }
    //if matches.get_flag("disassemble") { dump_disassemble(&obj, false, None, false, None); }
    //if matches.get_flag("disassemble-all") { dump_disassemble(&obj, true, None, false, None); }
    //if let Some(sym) = matches.get_one::<String>("disassemble-sym") { dump_disassemble(&obj, false, Some(sym), false, None); }
    //let source = matches.get_flag("source");
    //let source_comment = matches.get_one::<String>("source-comment");
    //if source || source_comment.is_some() { dump_disassemble(&obj, false, None, source, source_comment.map(|s| s.as_str())); }
    //if matches.get_flag("full-contents") { dump_full_contents(&obj); }
    //if matches.get_flag("decompress") { dump_decompress(&obj); }
    //if matches.get_flag("debugging") { dump_debugging(&obj, false); }
    //if matches.get_flag("debugging-tags") { dump_debugging(&obj, true); }
    //if matches.get_flag("stabs") { dump_stabs(&obj); }
    //if let Some(opt) = matches.get_one::<String>("dwarf") { dump_dwarf(&obj, Some(opt)); }
    //if matches.get_flag("process-links") { dump_process_links(&obj); }
    //if let Some(sect) = matches.get_one::<String>("ctf") { dump_ctf(&obj, Some(sect)); }
    //if let Some(sect) = matches.get_one::<String>("sframe") { dump_sframe(&obj, Some(sect)); }
    //if matches.get_flag("syms") { dump_symbols(&obj, false); }
    //if matches.get_flag("dynamic-syms") { dump_symbols(&obj, true); }
    //if matches.get_flag("reloc") { dump_reloc(&obj, false); }
    //if matches.get_flag("dynamic-reloc") { dump_reloc(&obj, true); }

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
        //&& !source
        //&& source_comment.is_none()
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
        //print_usage();
    }
    Ok(())
}