use std::io::{self, Result, stdin, Write, Seek, SeekFrom, Read};
use std::fs::{self, File, OpenOptions};

fn main() {
    let input_file_contents_result = get_input_file_contents();
    let input_file_contents : String;
    match input_file_contents_result {
        Ok(f) => {
            input_file_contents = f;
            println!("File opened successfully with contents: {}", input_file_contents);
        }
        Err(e) => {
            println!("Error opening file: {}. Exiting program.", e);
            return;
        }
    }

    let output_file_result = create_output_file();
    let mut output_file : File;

    match output_file_result {
        Ok(f) => {
            output_file = f;
            println!("File created successfully: {:?}", output_file);
        }
        Err(e) => {
            println!("Error creating file: {:?}. Exiting program.", e);
            return;
        }
    }

    let cipher : String = "0xexcellentParade".to_string();
    let encrypted_contents : Vec<u8> = encrpyt(input_file_contents.as_bytes(), cipher.as_bytes());
    let unencrypted_contents : Vec<u8> = encrpyt(&encrypted_contents, cipher.as_bytes());

    if unencrypted_contents == input_file_contents.as_bytes() {
        println!("Encryption and decryption successful. Unencrypted string match original contents. This is prior to writing and reading from the output file.");
    } else {
        println!("Encryption and decryption failed. Unencrypted string do not match original contents. This is prior to writing and reading from the output file. Exiting program.");
        return;
    }
    
    let write_output_result : Result<()> = output_file.write_all(&encrypted_contents);
    match write_output_result {
        Ok(_) => println!("Encrypted contents written to output file successfully."),
        Err(e) => {
            println!("Failed to write encrypted contents to output file: {:?}. Exiting program.", e);
            return;
        }
    }

    let output_file_seek_result : Result<u64> = output_file.seek(SeekFrom::Start(0));
    match output_file_seek_result {
        Ok(_) => println!("Output file seeked to start successfully."),
        Err(e) => {
            println!("Failed to seek to start of output file: {:?}. Exiting program.", e);
            return;
        }
    }

    let mut output_file_contents : Vec<u8> = Vec::new();
    let read_output_result : Result<usize> = output_file.read_to_end(&mut output_file_contents);
    match read_output_result {
        Ok(_) => println!("Output file read successfully with contents: {:?}", output_file_contents),
        Err(e) => {
            println!("Failed to read output file: {:?}. Exiting program.", e);
            return;
        }
    }

    let unencrypted_output_contents : Vec<u8> = encrpyt(&output_file_contents, cipher.as_bytes());
    if unencrypted_output_contents == input_file_contents.as_bytes() {
        println!("Encryption and decryption successful. Unencrypted string match original contents. This is after writing and reading from the output file.");
    } else {
        println!("Encryption and decryption failed. Unencrypted string do not match original contents. This is after writing and reading from the output file. Exiting program.");
        return;
    }



}

fn encrpyt(text : &[u8], key: &[u8]) -> Vec<u8> {
    let cipher : Vec<u8> = key.to_vec();
    let encrypted_contents : Vec<u8> = text.iter().enumerate().map(|(i, &c)| {
        let cipher_char = cipher[i % cipher.len()];
        let encrypted_char = (c as u8) ^ (cipher_char as u8);
        encrypted_char as u8
    }).collect();
    encrypted_contents
}

fn create_output_file() -> Result<File> {
    let mut output_file_path = String::from("data/encrypted/");
    println!("Enter path for output file. Note that paths start from data/encrypted/:");
    let output_file_path_result : io::Result<usize> = stdin().read_line(&mut output_file_path);
    match output_file_path_result {
        Ok(_) => {
            println!("Output File Path: {:?}", output_file_path.trim());
        }
        Err(e) => {
            println!("Cannot read input: {:?}", e);
            return Err(e);
        }
    }

    match OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(output_file_path.trim()) {
            Ok(file) => {
                println!("createOutputFile created file with path {:?} successfully", output_file_path.trim());
                Ok(file)
            }
            Err(e) => {
                println!("createOutputFile cannot create file with path {:?}: {:?}", output_file_path.trim(), e);
                Err(e)
            }
        }
}

fn get_input_file_contents() -> Result<String> {
    let mut input_file_path = String::from("data/raw/");
    println!("Enter path for file one. Note that paths start from data/raw/:");
    let input_file_path_result : io::Result<usize> = stdin().read_line(&mut input_file_path);
    match input_file_path_result {
        Ok(_) => {
            println!("File Path: {:?}", input_file_path.trim());
        }
        Err(e) => {
            println!("Cannot read input: {:?}", e);
            return Err(e);
        }
    }

    match fs::read_to_string(input_file_path.trim()) {
        Ok(content) => {
            println!("getInputFileContents opened file with path {:?} successfully", input_file_path.trim());
            Ok(content)
        }
        Err(e) => {
            println!("getInputFileContents cannot open file with path {:?}: {:?}", input_file_path.trim(), e);
            Err(e)
        }
    }
}
