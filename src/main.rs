use landscape::{maps::Chunk, modifiers::DiamondSquare, Modifier};
use rand::thread_rng;

fn main() {
    let mut chunk = Chunk::<f32>::with_edge_size(2048);

    let mut ds = DiamondSquare::new(thread_rng());
    ds.modify(&mut chunk);
    landscape::to_image(chunk.buffer, chunk.edge_size, "terrain.png").unwrap();
}
