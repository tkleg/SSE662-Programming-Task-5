use std::io;
use std::fs::File;

fn main() {
    let mut inputFilePath = String::from("data/raw/");
    println!("Enter path for file one. Note that paths start from data/raw/:");
    let inputFilePathResult : io::Result<usize> = io::stdin().read_line(&mut inputFilePath);
    match inputFilePathResult {
        Ok(_) => {
            println!("File Path: {}", inputFilePath.trim());
        }
        Err(e) => {
            println!("Cannot read input: {}", e);
            return;
        }
    }

    let f1 : File = match File::open(inputFilePath.trim()) {
        Ok(file) => {
            println!("File opened successfully");
            file
        }
        Err(e) => {
            println!("Cannot open file: {}. Path {}", e, inputFilePath.trim());
            return;
        }
    };
}
