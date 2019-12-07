pub mod maps;
pub mod modifiers;

pub use maps::Map;
pub use modifiers::Modifier;

use std::fs::File;

use image::{png::PNGEncoder, ColorType};
use num::{FromPrimitive, Num, ToPrimitive};

// TODO: move all this stuff to Map
fn normalize<T: Num + std::cmp::PartialOrd + Copy>(vec: &mut [T]) {
    let max = *vec
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    let min = *vec
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    let gap = max - min;
    for height in vec {
        *height = (*height - min) / gap;
    }
}

fn to_bytes<T: Num + ToPrimitive + FromPrimitive + std::cmp::PartialOrd + Copy>(
    map: &mut [T],
) -> Vec<u8> {
    normalize(map);
    map.iter()
        .map(|&value| {
            (value * T::from_u8(u8::max_value()).unwrap())
                .to_u8()
                .unwrap()
        })
        .collect()
}

// TODO: Move to separate module
pub fn to_image(mut height_map: Vec<f32>, edge_size: usize, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let encoder = PNGEncoder::new(file);
    let bytes = to_bytes(&mut height_map);
    encoder.encode(
        &bytes,
        edge_size as u32,
        edge_size as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
