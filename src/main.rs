use landscape::diamond_square;

fn main() {
    let map = diamond_square::generate(rand::thread_rng(), 2048);
    let map = map.scale(0u8, 255u8);
    landscape::to_image(&map, "terrain.png").unwrap();
}
