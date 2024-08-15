use crate::data::HeightMap;

pub fn print_height_map(height_map: &HeightMap) {
    for (i, height) in height_map.buf.iter().enumerate() {
        print!("{:.2}\t", height);
        if (i + 1) % height_map.width as usize == 0 {
            println!();
        }
    }
}
