use std::env;
use std::process;
use file_operation::Config;



fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {:?}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = file_operation::read_file(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
    
}


