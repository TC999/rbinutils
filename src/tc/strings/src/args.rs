pub struct Args {
    pub filename: String,
    pub min_len: usize,
}

impl Args {
    pub fn parse() -> Self {
        let mut min_len = 4usize;
        let args: Vec<String> = std::env::args().collect();

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
        Args {
            filename: file_arg.to_string(),
            min_len,
        }
    }
}