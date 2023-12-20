use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

fn read_file_1mb_exif(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut buffer = [0; 1024];

    // read exactly 1MB
    f.read_exact(&mut buffer)?;
    Ok(buffer.to_vec())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:x}", b).to_uppercase())
        .collect::<Vec<String>>()
        .join("")
}

struct JpegExifSegments<'a> {
    start_of_image: &'a [u8],
    exif_start: &'a [u8],
    tiff_header: &'a [u8],
    byte_order_endianness: &'a [u8],
    magic_number: &'a [u8],
    offset_to_ifd: &'a [u8],
    number_of_entries: &'a [u8],
    entries: &'a [u8],
 }

impl fmt::Display for JpegExifSegments<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_big_endian = self.is_bigendian();
        write!(f, 
            r"JpegExifSegments:
            big endian: {}
            endianness set correctly: {}
            ", 
            is_big_endian, 
            self.is_magic_number_42(is_big_endian)
        )
    }
}

impl<'a> JpegExifSegments<'a> {
   fn new(vec: &'a [u8]) -> Self {
       JpegExifSegments {
           start_of_image: &vec[0..2],
           exif_start: &vec[2..4],
           tiff_header: &vec[4..12],
           byte_order_endianness: &vec[12..14],
           magic_number:&vec[14..16],
           offset_to_ifd: &vec[16..20],
           number_of_entries: &vec[20..22],
           entries: &vec[22..],
       }
   }
   fn is_bigendian(&self) -> bool {
    match self.byte_order_endianness {
        &[77, 77] => {
            println!("Reading as little endian");
            false
        },
        &[73, 73] => {
            println!("Reading as big endian");
            true
        },
        _ => false,
    }
    }

    fn is_magic_number_42(&self, is_big_endian: bool) -> bool {
        // 42 magic number used to check endianness OK
        let magic_num = self.magic_number;
        if is_big_endian {
            magic_num[0] == 42
        } else {
            magic_num[1] == 42
        }
     }
     
     fn is_endianness_reading_ok(&self) -> Result<(), &'static str> {
        // Everything later will be garbage if endianness is wrong
        let is_big_endian = self.is_bigendian();
        if self.is_magic_number_42(is_big_endian) {
            Ok(())
        } else {
            Err("Endianness is not correct")
        }
     }
      
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../vista.jpg";
    // let jpeg_bytes = read_file_to_bytes_vec(path)?;
    let jpeg_bytes = read_file_1mb_exif(path)?;
    let jpeg_segments = JpegExifSegments::new(&jpeg_bytes);
    println!("{}", jpeg_segments);
    Ok(())
}
