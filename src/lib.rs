use std::error::Error;
use std::fs;

pub struct Config{
    pub query: String,
    pub file_path: String
}

impl Config{
    // &'staic str      string slice in static storage
    // 好处在于，不需要 heap allocation，效率更高
    pub fn build(args: &[String]) -> Result<Config, &'static str>{      // 注意：没有 &self，是 class method
        // Bad choice of giving panic for exception handling
        /*if(args.len()<3){
            panic!("not enough arguments");
        }*/
        if (args.len() < 3){
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
        // Return 出来的东西是一个 result，需要 unwrap。方法：unwrap_or_else
    }
}

pub fn run(config: Config)-> Result<(), Box<dyn Error>> {   // () 是 unit type; Box<dyn Error> 是 Boxed error type，拥有 error trait，但是丢失了信息。
    let content = fs::read_to_string(config.file_path)?;
    println!("With a content:\n{content}");
    Ok(())
}