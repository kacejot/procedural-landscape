pub mod diamond_square;
pub mod height_map;
pub mod map;

use std::fs::File;

use image::{png::PNGEncoder, ColorType};
use num::{FromPrimitive, Num, ToPrimitive};

use height_map::HeightMap;

// TODO: Move to separate module
pub fn to_image(height_map: &HeightMap<u8>, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let encoder = PNGEncoder::new(file);
    encoder.encode(
        &height_map.buffer,
        height_map.edge_size as u32,
        height_map.edge_size as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
