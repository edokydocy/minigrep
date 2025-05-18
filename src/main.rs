use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();  // args() returns an iterator; .collect() enum iterators
    dbg!(&args);    // cannot use "args" here. Moved

    // save variables
    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for query: {query}");
    println!("In file {file_path}");

    // print out contents
    let content = fs::read_to_string(file_path)
            .expect("should be able to read the file");
    
    println!("With a content:\n{content}");
}
