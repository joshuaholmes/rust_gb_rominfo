// Abstraction for dealing with ROM files.

//
// Author: Joshua Holmes
//

use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use std::str;

use util;

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
	/// Takes in a file path and returns a Rom
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

	    // if the ROM isn't even large enough to hold a header, it's invalid
	    if buf.len() <= 0x014F {
	    	return Err(RomLoadError::FormatError)
	    }

	    // multi-byte values, copy as a chunk from the buffer
	    let mut entry_point = [0u8; 4];
	    let mut nintendo_logo = [0u8; 48];
		let mut title = [0u8; 11];
		let mut manufacturer_code = [0u8; 4];
		let mut new_licensee_code = [0u8; 2];

		try!(file.seek(SeekFrom::Start(0x0100)));
		try!(util::read_to_buf(&mut entry_point, file));
		try!(util::read_to_buf(&mut nintendo_logo, file));
		try!(util::read_to_buf(&mut title, file));
		try!(util::read_to_buf(&mut manufacturer_code, file));
		try!(file.seek(SeekFrom::Start(0x0144)));
		try!(util::read_to_buf(&mut new_licensee_code, file));

		// single-byte values, read from the buffer directly
		let cgb_flag = buf[0x0143];
		let sgb_flag = buf[0x0146];
	    let cartridge_type = buf[0x0147];
	    let rom_size_flag = buf[0x0148];
	    let ram_size_flag = buf[0x0149];
	    let destination_code = buf[0x014A];
	    let old_licensee_code = buf[0x014B];
	    let mask_rom_version_number = buf[0x014C];
	    let header_checksum = buf[0x014D];
	    let global_checksum = ((buf[0x014E] as u16) << 8) | (buf[0x014F] as u16); 

	    Ok(Rom {
	    	entry_point: entry_point,
	    	nintendo_logo: nintendo_logo,
	    	title: title,
	    	manufacturer_code: manufacturer_code,
	    	cgb_flag: cgb_flag,
	    	new_licensee_code: new_licensee_code,
	    	sgb_flag: sgb_flag,
	    	cartridge_type: cartridge_type,
	    	rom_size_flag: rom_size_flag,
	    	ram_size_flag: ram_size_flag,
	    	destination_code: destination_code,
	    	old_licensee_code: old_licensee_code,
	    	mask_rom_version_number: mask_rom_version_number,
	    	header_checksum: header_checksum,
	    	global_checksum: global_checksum,
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

	    for i in 0x0134..0x014D {
	        calculated_header_checksum = calculated_header_checksum - (self.rom_data[i] as u16) - 1;
	    }

	    (calculated_header_checksum as u8) == self.header_checksum
	}

	/// Says whether the global checksum is valid
	pub fn is_global_checksum_valid(&self) -> bool {
		let mut calculated_global_checksum: u16 = 0;

	    for (i, x) in self.rom_data.iter().enumerate() {
	        if i != 0x014E && i != 0x014F {
	            calculated_global_checksum += *x as u16;
	        }
	    }

	    calculated_global_checksum == self.global_checksum
	}
}