use std::convert::TryInto;
use std::io::Cursor;

use afl::fuzz;
use mp4::Mp4Reader;

fn main() {
    fuzz!(|bytes: &[u8]| {
        let reader = Cursor::new(bytes);
        let _ = Mp4Reader::read_header(reader, bytes.len().try_into().unwrap());
    });
}
