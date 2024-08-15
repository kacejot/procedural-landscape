use landscape::{modifiers::DiamondSquare, data::{HeightMap, utils}};

fn main() {
    // TODO: make seed configurable
    // TODO: mutators should have a way to track their progress
    let mut ds = DiamondSquare::new(rand::thread_rng());
    let mut map = HeightMap::with_multiple_chunks(64, 3, 2);
    ds.modify(&mut map);

    let buf: Vec<u8> = utils::to_buf(&map, 0u8, 255u8);
    landscape::to_image(&buf, map.width, map.height, "terrain.png").unwrap();
}
