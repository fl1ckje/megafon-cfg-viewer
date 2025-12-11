use encoding::{DecoderTrap, Encoding, all::KOI8_R};
use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("C:\\projects\\megafon-cfg-viewer\\test_data\\screen041.conf")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let decoded_content = match KOI8_R.decode(&buffer, DecoderTrap::Strict) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to decode file: {}", e);
            return Ok(());
        }
    };
    println!("File content: {}", decoded_content);
    Ok(())
}
