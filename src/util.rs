//
// Author: Joshua Holmes
//

use std::io::{self, Read};
use std::str;

/// Reads until the buffer is filled or the reader signals EOF
/// Borrowed from https://github.com/pcwalton/sprocketnes/blob/master/src/util.rs
pub fn read_to_buf(mut buf: &mut [u8], rd: &mut Read) -> io::Result<()> {
    let mut total = 0;
    while total < buf.len() {
        let count = try!(rd.read(&mut buf[total..]));
        if count == 0 {
            // Buffer not yet filled, but EOF reached
            return Err(io::Error::new(io::ErrorKind::Other, "eof reached prematurely"))
        }
        total += count;
    }

    Ok(())
}

/// Converts the given u8 slice into a UTF-8 string
pub fn bytes_to_utf8_string(bytes: &[u8]) -> &str {
    match str::from_utf8(bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}