pub struct Args {
    pub filename: String,
    pub min_len: usize,        // 普通扫描长度
    pub scan_object: bool,     // 是否数据段扫描
    pub object_min_len: usize, // 数据段扫描长度
}

impl Args {
    pub fn parse() -> Self {
        let mut min_len = 4usize;
        let mut scan_object = false;
        let mut object_min_len = 4usize;
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            eprintln!("用法: strings <filename> [-n min_len] [-d min_len]");
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
                "-d" => {
                    scan_object = true;
                    if i + 1 < args.len() && args[i + 1].chars().all(|c| c.is_ascii_digit()) {
                        object_min_len = args[i + 1].parse().unwrap_or(4);
                        i += 1;
                    }
                }
                filename => {
                    file_arg = filename;
                }
            }
            i += 1;
        }
        Args {
            filename: file_arg.to_string(),
            min_len,
            scan_object,
            object_min_len,
        }
    }
}