// Main driver of the program.

//
// Author: Joshua Holmes
//

use std::str;
use std::env;

mod rom;
mod util;

use rom::Rom;

fn main() {
    // get the filename from the command args and load the ROM
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];

    let rom_file: Rom = match Rom::from_file_path(filename) {
        Err(e) => panic!("Failed to load ROM file. Error message: {:?}", e),
        Ok(rom) => rom,
    };

    // print out all the info
    println!("\nTitle: {}", rom_file.get_title_string());
    println!("Manufacturer code: {}", rom_file.get_manufacturer_code_string());
    println!("CGB flag: {:#X}", rom_file.cgb_flag);
    println!("New licensee code: {}", rom_file.get_new_licensee_code_string());
    println!("SGB flag: {:#X}", rom_file.sgb_flag);
    println!("Cartridge type: {:#X}", rom_file.cartridge_type);
    println!("ROM size flag: {:#X}", rom_file.rom_size_flag);
    println!("RAM size flag: {:#X}", rom_file.ram_size_flag);
    println!("Destination code: {:#X}", rom_file.destination_code);
    println!("Old licensee code: {:#X}", rom_file.old_licensee_code);
    println!("Mask ROM version number: {:#X}", rom_file.mask_rom_version_number);
    println!("Header checksum: {:#X} (valid: {})", rom_file.header_checksum, rom_file.is_header_checksum_valid());
    println!("Global checksum: {:#X} (valid: {})", rom_file.global_checksum, rom_file.is_global_checksum_valid());
}