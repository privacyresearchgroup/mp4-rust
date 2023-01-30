use std::convert::TryInto;
use std::fs::File;

use mp4::Mp4Reader;

// Alternate entry point, for ease of debugging. The main entry point (src/main.rs) aborts on panic
// for AFL++ reasons, but that makes it hard to tell _why_ it panicked.

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file = File::open(&path).unwrap();
    let len = file.metadata().unwrap().len();
    let _ = dbg!(Mp4Reader::read_header(file, len.try_into().unwrap()));
}
