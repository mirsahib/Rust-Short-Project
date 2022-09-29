use std::error::Error;
use std::io;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum FileOperationError{
    InvalidOperator,
    FileNotWritable,
    Interrupted,
}


fn main() -> Result<(),FileOperationError> {
    println!("FileOperation \n 1) Read file \n 2) Write file \n 0) Exit program");
    let mut op_type = String::new();

    loop {
        io::stdin()
            .read_line(&mut op_type)
            .expect("Failed to read line");

        let op_type: u32 = match op_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if op_type == 1 {
            println!("please enter file path");
            let mut file_path = String::new();
            io::stdin()
                .read_line(&mut file_path)
                .expect("Failed to read file path");
            // read file contents
        } else if op_type == 2 {
            println!("please enter file name");
            let mut file_name = String::new();
            io::stdin()
                .read_line(&mut file_name)
                .expect("Failed to read file name");
            // write file contents

            let mut file:File = match File::create(&file_name) {
                Ok(val) => val,
                Err(err) => return Err(FileOperationError::FileNotWritable),
            };
            match file.write_all(b"Hello, world!"){
                Ok(val) => val,
                Err(err) => println!("{:?}", err)
            };
            break;
        } else if op_type == 0 {
            break;
        } else {
            return Err(FileOperationError::InvalidOperator);
        }
    }
    Ok(())
    // Read file operation
}
