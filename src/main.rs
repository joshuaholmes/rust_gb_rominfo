use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;

fn main() {
    // get a file handle to our ROM
    let path = Path::new("rom.gb");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}. Error message: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // read the ROM into a buffer
    let mut buf = Vec::new();

    match file.read_to_end(&mut buf) {
        Err(why) => panic!("Couldn't read {}. Error message: {}", display, Error::description(&why)),
        Ok(_) => println!("{} read successfully. Length: {}", display, buf.len()),
    }

    // start reading information about the ROM

    // title
    let title_bytes = &buf[0x0134..0x013F];
    let title = match str::from_utf8(title_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("\nTitle bytes: {:?}", title_bytes);
    println!("Title: {}", title);

    // manufacturer code
    let manufacturer_code_bytes = &buf[0x013F..0x0143];
    let manufacturer_code = match str::from_utf8(manufacturer_code_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("\nManufacturer code bytes: {:?}", manufacturer_code_bytes);
    println!("Manufacturer code: {}", manufacturer_code);

    // CGB flag
    let cgb_flag = buf[0x0143];
    println!("\nCGB flag: 0x{:X}", cgb_flag);

    // new licensee code
    let new_licensee_code_bytes = &buf[0x0144..0x0146];
    let new_licensee_code = match str::from_utf8(new_licensee_code_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("\nNew licensee code bytes: {:?}", new_licensee_code_bytes);
    println!("New licensee code: {}", new_licensee_code);
}