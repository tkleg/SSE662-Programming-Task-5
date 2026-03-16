use std::io::{self, Error, Result, stdin};
use std::fs::{self, File};
use std::str::Chars;

fn main() {
    let inputFileContentsResult = getInputFileContents();
    let inputFileContents : String;
    match inputFileContentsResult {
        Ok(f) => {
            inputFileContents = f;
            println!("File opened successfully: {:?}", inputFileContents);
        }
        Err(e) => {
            println!("Error opening file: {}. Exiting program.", e);
            return;
        }
    }

    let cipher : Chars = "0xexcellentParade".chars();

    let outputFileResult = createOutputFile();
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

fn createOutputFile() -> io::Result<File> {
    let mut outputFilePath = String::from("data/encrypted/");
    println!("Enter path for output file. Note that paths start from data/encrypted/:");
    let outputFilePathResult : io::Result<usize> = stdin().read_line(&mut outputFilePath);
    match outputFilePathResult {
        Ok(_) => {
            println!("Output File Path: {}", outputFilePath.trim());
        }
        Err(e) => {
            println!("Cannot read input: {}", e);
            return Err(e);
        }
    }

    match File::create(outputFilePath.trim()) {
        Ok(file) => {
            println!("createOutputFile created file with path {} successfully", outputFilePath.trim());
            Ok(file)
        }
        Err(e) => {
            println!("createOutputFile cannot create file with path {}: {}", outputFilePath.trim(), e);
            Err(e)
        }
    }
}

fn getInputFileContents() -> io::Result<String> {
    let mut inputFilePath = String::from("data/raw/");
    println!("Enter path for file one. Note that paths start from data/raw/:");
    let inputFilePathResult : io::Result<usize> = stdin().read_line(&mut inputFilePath);
    match inputFilePathResult {
        Ok(_) => {
            println!("File Path: {}", inputFilePath.trim());
        }
        Err(e) => {
            println!("Cannot read input: {}", e);
            return Err(e);
        }
    }

    match fs::read_to_string(inputFilePath.trim()) {
        Ok(content) => {
            println!("getInputFileContents opened file with path {} successfully", inputFilePath.trim());
            Ok(content)
        }
        Err(e) => {
            println!("getInputFileContents cannot open file with path {}: {}", inputFilePath.trim(), e);
            Err(e)
        }
    }
}
