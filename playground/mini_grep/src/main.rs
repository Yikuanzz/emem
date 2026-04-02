use mini_grep::Config;
use std::env;
use std::process;

fn main() {
    // 1. 获取参数迭代器并传给 Config::build
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 2. 运行主逻辑
    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}