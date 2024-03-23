use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use sha2::Digest;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: tth <text> or tth <file>");
        return Ok(());
    }

    let text_to_hash = if Path::new(&args[1]).is_file() {
        // Read from a file
        let file = File::open(&args[1])?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        content
    } else {
        // Use the text directly
        args[1].clone()
    };

    let mut hasher = sha2::Sha256::new();
    hasher.update(text_to_hash.as_bytes());
    let hash = hasher.finalize();

    // Write the hash to "hash.txt"
    File::create("hash.txt")?.write_all(&hash.as_slice())?;

    println!("Hash written to hash.txt");
    Ok(())
}

