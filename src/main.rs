use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);    // cannot use "args" here. Moved

    // save variables
    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for query: {query}");
    println!("In file {file_path}");
}
