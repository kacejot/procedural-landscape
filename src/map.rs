pub mod chunk;

pub use chunk::Chunk;

use num::Num;

pub trait Map
where
    Self::ItemType: Num + Copy,
{
    type ItemType;

    fn edge_size(&self) -> usize;

    fn at(&self, x: usize, y: usize) -> Self::ItemType;
    fn at_mut(&mut self, x: usize, y: usize) -> &mut Self::ItemType;
}
