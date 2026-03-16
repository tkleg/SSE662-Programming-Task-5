use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a string:");
    let result : io::Result<usize> = io::stdin().read_line(&mut input);
    match result {
        Ok(_) => {
            println!("string: {}", input.trim());
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}
