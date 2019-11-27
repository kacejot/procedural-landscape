mod map;
mod modifier;
mod util;

use rand::thread_rng;

use map::chunk::Chunk;
use modifier::{generator::DiamondSquare, Modifier};

fn main() {
    let mut chunk = Chunk::<f32>::with_edge_size(4);

    let roughness = 1.0;
    let mut ds = DiamondSquare::new(thread_rng(), roughness);
    ds.modify(&mut chunk);
}
