use std::env;
use std::env::Args;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();  // args() returns an iterator; .collect() enum iterators
    dbg!(&args);    // cannot use "args" here. Moved

    // save variables
    // Config::build 的结果需要 unwrap。这里是 unwrap_or_else，
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for query: {}", config.query);
    println!("In file {}", config.file_path);

    // print out contents
    run(config);
}

struct Config{
    query: String,
    file_path: String
}

impl Config{
    // &'staic str      string slice in static storage
    // 好处在于，不需要 heap allocation，效率更高
    fn build(args: &[String]) -> Result<Config, &'static str>{      // 注意：没有 &self，是 class method
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

fn run(config: Config){
    let content = fs::read_to_string(config.file_path)
        .expect("Should be able to read the file.");
    println!("With a content:\n{content}");
}