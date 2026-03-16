use std::io;
use std::fs::File;

fn main() {
    let file = getInputFile();
    match file {
        Ok(f) => {
            println!("File opened successfully: {:?}", f);
        }
        Err(e) => {
            println!("Error opening file: {}", e);
        }
    }
}

fn getInputFile() -> Result<File, io::Error> {
    let mut inputFilePath = String::from("data/raw/");
    println!("Enter path for file one. Note that paths start from data/raw/:");
    let inputFilePathResult : io::Result<usize> = io::stdin().read_line(&mut inputFilePath);
    match inputFilePathResult {
        Ok(_) => {
            println!("File Path: {}", inputFilePath.trim());
        }
        Err(e) => {
            println!("Cannot read input: {}", e);
            return Err(e);
        }
    }

    let f1 : File = match File::open(inputFilePath.trim()) {
        Ok(file) => {
            println!("getInputFile opened file with path {} successfully", inputFilePath.trim());
            return Ok(file);
        }
        Err(e) => {
            println!("getInputFile cannot open file with path {}: {}", inputFilePath.trim(), e);
            return Err(e);
        }
    };
}
