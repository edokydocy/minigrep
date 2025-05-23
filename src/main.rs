use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::Config;

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
    if let Err(e) = minigrep::run(config) {   // Err(e) receives Result<> to make "e" receive Box<> is like destructuring
        println!("Application Error: {e}");
        process::exit(1);
    }
}