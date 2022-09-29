use std::error::Error;
use std::fs;


#[derive(Debug)]
pub enum FileOperationError {
    MissingArgument,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, FileOperationError> {
        if args.len() < 3 {
            return Err(FileOperationError::MissingArgument);
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn read_file(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
