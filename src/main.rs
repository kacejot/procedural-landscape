use landscape::{maps::Chunk, modifiers::DiamondSquare, Modifier};
use rand::thread_rng;

fn main() {
    let mut chunk = Chunk::<f32>::with_edge_size(512);

    let with_flat_border = true;
    let mut ds = DiamondSquare::new(thread_rng(), with_flat_border);
    ds.modify(&mut chunk);
    
    landscape::to_image(chunk.buffer, chunk.edge_size, "terrain.png").unwrap();
}
