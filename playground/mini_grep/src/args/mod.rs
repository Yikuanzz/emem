use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{
        // 第 1 个是参数名，跳过
        args.next();
        
        // 第 2 个参数 query
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // 第 3 个参数 filename
        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // 检查环境变量
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{
            query,
            filename,
            case_sensitive,
        })
    }
}