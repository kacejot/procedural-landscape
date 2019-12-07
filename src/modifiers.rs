mod diamond_square;
pub use diamond_square::DiamondSquare;

use std::iter::Sum;

use crate::Map;

pub trait Modifier {
    fn modify<M>(&mut self, map: &mut M)
    where
        M: Map,
        M::ItemType: Sum + From<f32>;
}
