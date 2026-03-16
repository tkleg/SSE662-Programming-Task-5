use std::io;
use std::fs::File;
use std::str;

fn main() {
    let inputFileResult = getInputFile();
    let inputFile : File;
    match inputFileResult {
        Ok(f) => {
            inputFile = f;
            println!("File opened successfully: {:?}", inputFile);
        }
        Err(e) => {
            println!("Error opening file: {}. Exiting program.", e);
            return;
        }
    }

    let cipher : str::Chars = "0xexcellentParade".chars();

    let outputFileResult = getOutputFile();
    let outputFile : File;

    match outputFileResult {
        Ok(f) => {
            outputFile = f;
            println!("File created successfully: {:?}", outputFile);
        }
        Err(e) => {
            println!("Error creating file: {}. Exiting program.", e);
            return;
        }
    }
}

fn getOutputFile() -> Result<File, io::Error> {
    let mut outputFilePath = String::from("data/encrypted/");
    println!("Enter path for output file. Note that paths start from data/encrypted/:");
    let outputFilePathResult : io::Result<usize> = io::stdin().read_line(&mut outputFilePath);
    match outputFilePathResult {
        Ok(_) => {
            println!("Output File Path: {}", outputFilePath.trim());
        }
        Err(e) => {
            println!("Cannot read input: {}", e);
            return Err(e);
        }
    }

    let f2 : File = match File::create(outputFilePath.trim()) {
        Ok(file) => {
            println!("getOutputFile created file with path {} successfully", outputFilePath.trim());
            return Ok(file);
        }
        Err(e) => {
            println!("getOutputFile cannot create file with path {}: {}", outputFilePath.trim(), e);
            return Err(e);
        }
    };
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
