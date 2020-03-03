extern crate flate2;

use std::io::{self, SeekFrom};
use std::io::prelude::*;

use flate2::{Compression, FlateReadExt};
use flate2::write::ZlibEncoder;
use flate2::write::ZlibDecoder;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read};

fn main() {
    let bytes = b"I have a dream taht one day this nation will rise up,\
    and live out the true meaning of its creed";
    println!("Original: {:?}", bytes.as_ref());
    let encoded = encode_bytes(bytes.as_ref()).expect("Failed to encode bytes");
    println!("Decoded: {:?}", decoded);
}