pub mod diamond_square;
pub mod height_map;

use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};
use std::fs::File;

// TODO: Move to separate module
pub fn to_image(buf: &Vec<u8>, edge_size: usize, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let encoder = PngEncoder::new(file);

    match encoder.write_image(
        buf,
        edge_size as u32,
        edge_size as u32,
        ExtendedColorType::L8,
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}
