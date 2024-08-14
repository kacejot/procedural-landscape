use landscape::{diamond_square, height_map::to_buf};

fn main() {
    let map = diamond_square::generate(rand::thread_rng(), 2048);
    let buf: Vec<u8> = to_buf(&map, 0u8, 255u8);
    landscape::to_image(&buf, map.edge_size, "terrain.png").unwrap();
}
