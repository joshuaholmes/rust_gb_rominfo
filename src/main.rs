use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // get a file handle to our ROM
    let path = Path::new("rom.gb");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            panic!("Couldn't open {}. Error message: {}", display, Error::description(&why))
        },
        Ok(file) => {
            println!("File handle to the ROM opened successfully."); 
            file
        },
    };

    // read the ROM into a buffer
    let mut buf = Vec::new();

    match file.read_to_end(&mut buf) {
        Err(why) => {
            panic!("Couldn't read {}. Error message: {}", display, Error::description(&why))
        },
        Ok(_) => {
            println!("ROM file read successfully. Length: {}", buf.len())
        },
    }

    // start reading information about the ROM
}