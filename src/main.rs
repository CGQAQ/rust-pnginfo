#![allow(dead_code)]

use std::env;
use std::io::Read;
use std::mem;

#[derive(Clone, Copy, Debug)]
#[repr(packed(1))]
struct PngIhdr {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(packed(1))]
struct PngIheader {
    length: u32,
    chunk_type: [u8; 4],
    data: PngIhdr,
    crc: u32,
}

const SIZEOF_IHEADER: usize = mem::size_of::<PngIheader>();

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file_path = args[1].clone();

    let mut file = match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Error reading file");

    let mut iheader: PngIheader = unsafe {
        mem::transmute_copy(&*(buf[8..SIZEOF_IHEADER + 8].as_ptr() as *const [u8; SIZEOF_IHEADER]))
    };

    iheader.length = iheader.length.swap_bytes();
    iheader.data.width = iheader.data.width.swap_bytes();
    iheader.data.height = iheader.data.height.swap_bytes();

    println!("{:#?}", iheader);
}
