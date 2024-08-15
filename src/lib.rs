pub mod data;
pub mod modifiers;

use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};
use std::fs::File;

// TODO: Move to separate module
pub fn to_image(buf: &Vec<u8>, width: u32, height: u32, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let encoder = PngEncoder::new(file);

    match encoder.write_image(buf, width, height, ExtendedColorType::L8) {
        Ok(_) => Ok(()),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}
