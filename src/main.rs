use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    // get a file handle to our ROM
    let filename = &args[1];
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}. Error message: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // read the ROM into a buffer
    let mut buf = Vec::new();

    match file.read_to_end(&mut buf) {
        Err(why) => panic!("Couldn't read {}. Error message: {}", display, Error::description(&why)),
        Ok(_) => println!("{} read successfully. ROM size: {} bytes", display, buf.len()),
    }

    // read in all the information from the ROM header
    let nintendo_logo = &buf[0x104..0x134];

    let title_bytes = &buf[0x0134..0x013F];
    let title = match str::from_utf8(title_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let manufacturer_code_bytes = &buf[0x013F..0x0143];
    let manufacturer_code = match str::from_utf8(manufacturer_code_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let cgb_flag = buf[0x0143];

    let new_licensee_code_bytes = &buf[0x0144..0x0146];
    let new_licensee_code = match str::from_utf8(new_licensee_code_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let sgb_flag = buf[0x0146];
    let cartridge_type = buf[0x0147];
    let rom_size_flag = buf[0x0148];
    let ram_size_flag = buf[0x0149];
    let destination_code = buf[0x014A];
    let old_licensee_code = buf[0x014B];
    let mask_rom_version_number = buf[0x014C];
    let header_checksum = buf[0x014D];
    let global_checksum = ((buf[0x014E] as u16) << 8) | (buf[0x014F] as u16); 

    // calculate header checksum
    let mut calculated_header_checksum: u16 = 0;
    for i in 0x0134..0x014D {
        calculated_header_checksum = calculated_header_checksum - (buf[i] as u16) - 1;
    }
    let calculated_header_checksum = calculated_header_checksum as u8;

    // calculate global checksum
    let mut calculated_global_checksum: u16 = 0;
    for (i, x) in buf.iter().enumerate() {
        if i != 0x014E && i != 0x014F {
            calculated_global_checksum += *x as u16;
        }
    }

    // print everything out
    println!("\nNintendo logo: ");
    for i in 0..3 {
        let start = i * 16;

        for x in &nintendo_logo[start..start+16] {
            print!("{:02X} ", x);
        }

        println!("");
    }

    println!("\nTitle: {}", title);
    println!("Manufacturer code: {}", manufacturer_code);
    println!("CGB flag: {:#X}", cgb_flag);
    println!("New licensee code: {}", new_licensee_code);
    println!("SGB flag: {:#X}", sgb_flag);
    println!("Cartridge type: {:#X}", cartridge_type);
    println!("ROM size flag: {:#X}", rom_size_flag);
    println!("RAM size flag: {:#X}", ram_size_flag);
    println!("Destination code: {:#X}", destination_code);
    println!("Old licensee code: {:#X}", old_licensee_code);
    println!("Mask ROM version number: {:#X}", mask_rom_version_number);
    println!("Header checksum: {:#X} (valid: {})", header_checksum, header_checksum == calculated_header_checksum);
    println!("Global checksum: {:#X} (valid: {})", global_checksum, global_checksum == calculated_global_checksum);
}