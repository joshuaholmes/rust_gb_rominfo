// Abstraction for dealing with ROM files.

//
// Author: Joshua Holmes
//

use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str;

use util;

// Header address constants
const ENTRY_POINT_ADDR: usize = 0x0100;
const NINTENDO_LOGO_ADDR: usize = 0x0104;
const TITLE_ADDR: usize = 0x0134;
const MANUFACTURER_CODE_ADDR: usize = 0x013F;
const CGB_FLAG_ADDR: usize = 0x0143;
const NEW_LICENSEE_CODE_ADDR: usize = 0x0144;
const SGB_FLAG_ADDR: usize = 0x0146;
const CARTRIDGE_TYPE_ADDR: usize = 0x0147;
const ROM_SIZE_ADDR: usize = 0x0148;
const RAM_SIZE_ADDR: usize = 0x0149;
const DESTINATION_CODE_ADDR: usize = 0x014A;
const OLD_LICENSEE_CODE_ADDR: usize = 0x014B;
const MASK_ROM_VERSION_NUMBER_ADDR: usize = 0x014C;
const HEADER_CHECKSUM_ADDR: usize = 0x014D;
const GLOBAL_CHECKSUM_ADDR: usize = 0x014E;

/// RomLoadError borrowed from sprocketnes
#[derive(Debug)]
pub enum RomLoadError {
    /// IO error while reading the ROM image
    IoError(io::Error),
    /// The ROM image has an invalid format
    FormatError,
}

impl From<io::Error> for RomLoadError {
    fn from(err: io::Error) -> Self {
        RomLoadError::IoError(err)
    }
}

/// Structure to represent an actual ROM file
pub struct Rom {
	pub entry_point: [u8; 4],
	pub nintendo_logo: [u8; 48],
	pub title: [u8; 11],
	pub manufacturer_code: [u8; 4],
	pub cgb_flag: u8,
	pub new_licensee_code: [u8; 2],
	pub sgb_flag: u8,
	pub cartridge_type: u8,
	pub rom_size_flag: u8,
	pub ram_size_flag: u8,
	pub destination_code: u8,
	pub old_licensee_code: u8,
	pub mask_rom_version_number: u8,
	pub header_checksum: u8,
	pub global_checksum: u16,
	pub rom_data: Vec<u8>,
}

impl Rom {
	/// Takes in a file path string and returns a Rom
	pub fn from_file_path(filepath: &str) -> Result<Rom, RomLoadError> {
	    let path = Path::new(filepath);

	    let mut file = match File::open(&path) {
	        Err(e) => panic!("Couldn't open ROM file. Error message: {}", Error::description(&e)),
	        Ok(file) => file,
	    };

	    Rom::from_file(&mut file)
	}

	/// Takes in a File object and reads the data into a Rom structure
	pub fn from_file(file: &mut File) -> Result<Rom, RomLoadError> {
		// read the ROM into a buffer
	    let mut buf = Vec::new();

	    match file.read_to_end(&mut buf) {
	        Err(e) => panic!("Couldn't read ROM file. Error message: {}", Error::description(&e)),
	        Ok(_) => (),
	    }

	    // if the ROM size is less than or equal to the size needed to simply 
	    // store the cartridge header, then it's invalid
	    if buf.len() <= GLOBAL_CHECKSUM_ADDR + 1 {
	    	return Err(RomLoadError::FormatError)
	    }

	    // read the multi-byte values into our buffers
	    let mut entry_point = [0u8; 4];
	    let mut nintendo_logo = [0u8; 48];
		let mut title = [0u8; 11];
		let mut manufacturer_code = [0u8; 4];

		util::get_subarray_of_vector(&mut entry_point, &buf, ENTRY_POINT_ADDR);
		util::get_subarray_of_vector(&mut nintendo_logo, &buf, NINTENDO_LOGO_ADDR);
		util::get_subarray_of_vector(&mut title, &buf, TITLE_ADDR);
		util::get_subarray_of_vector(&mut manufacturer_code, &buf, MANUFACTURER_CODE_ADDR);

	    Ok(Rom {
	    	entry_point: entry_point,
	    	nintendo_logo: nintendo_logo,
	    	title: title,
	    	manufacturer_code: manufacturer_code,
	    	new_licensee_code: [buf[NEW_LICENSEE_CODE_ADDR], buf[NEW_LICENSEE_CODE_ADDR + 1]],
	    	cgb_flag: buf[CGB_FLAG_ADDR],
	    	sgb_flag: buf[SGB_FLAG_ADDR],
	    	cartridge_type: buf[CARTRIDGE_TYPE_ADDR],
	    	rom_size_flag: buf[ROM_SIZE_ADDR],
	    	ram_size_flag: buf[RAM_SIZE_ADDR],
	    	destination_code: buf[DESTINATION_CODE_ADDR],
	    	old_licensee_code: buf[OLD_LICENSEE_CODE_ADDR],
	    	mask_rom_version_number: buf[MASK_ROM_VERSION_NUMBER_ADDR],
	    	header_checksum: buf[HEADER_CHECKSUM_ADDR],
	    	global_checksum: ((buf[GLOBAL_CHECKSUM_ADDR] as u16) << 8) | (buf[GLOBAL_CHECKSUM_ADDR + 1] as u16),
	    	rom_data: buf,
	    })
	}

	/// Gets a string representation of the ROM title
	pub fn get_title_string(&self) -> &str {
		util::bytes_to_utf8_string(&self.title)
	}

	/// Gets a string represention of the manufacturer code
	pub fn get_manufacturer_code_string(&self) -> &str {
		util::bytes_to_utf8_string(&self.manufacturer_code)
	}

	/// Gets a string representation of the new licensee code
	pub fn get_new_licensee_code_string(&self) -> &str {
		util::bytes_to_utf8_string(&self.new_licensee_code)
	}

	/// Says whether the header checksum is valid
	pub fn is_header_checksum_valid(&self) -> bool {
		let mut calculated_header_checksum: u16 = 0;

	    for i in TITLE_ADDR..HEADER_CHECKSUM_ADDR {
	        calculated_header_checksum = calculated_header_checksum - (self.rom_data[i] as u16) - 1;
	    }

	    (calculated_header_checksum as u8) == self.header_checksum
	}

	/// Says whether the global checksum is valid
	pub fn is_global_checksum_valid(&self) -> bool {
		let mut calculated_global_checksum: u16 = 0;

	    for (i, x) in self.rom_data.iter().enumerate() {
	        if i != GLOBAL_CHECKSUM_ADDR && i != (GLOBAL_CHECKSUM_ADDR + 1) {
	            calculated_global_checksum += *x as u16;
	        }
	    }

	    calculated_global_checksum == self.global_checksum
	}
}