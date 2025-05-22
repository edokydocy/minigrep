use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();  // args() returns an iterator; .collect() enum iterators
    dbg!(&args);    // cannot use "args" here. Moved

    // save variables
    let config = parse_config(&args);
    
    println!("Searching for query: {}", config.query);
    println!("In file {}", config.file_path);

    // print out contents
    let content = fs::read_to_string(config.file_path)
            .expect("should be able to read the file");
    
    println!("With a content:\n{content}");
}

struct Config{
    query: String,
    file_path: String
}

fn parse_config(args: &[String]) -> Config{
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config{query, file_path}
}