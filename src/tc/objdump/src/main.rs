mod cmd;
mod display;
mod dispatch;
mod feature;

use crate::cmd::parse_args;
use std::io;

fn main() -> io::Result<()> {
    let matches = parse_args();

    // 参数分发逻辑
    // 例如：if matches.archive_headers { dump_archive_headers(...) }
    // 其它分发同原来 main 函数
    Ok(())
}