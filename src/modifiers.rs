mod diamond_square;

pub use diamond_square::DiamondSquare;

use crate::Map;
use num::FromPrimitive;
use std::iter::Sum;

pub trait Modifier {
    fn modify<M>(&mut self, map: &mut M)
    where
        M: Map,
        M::ItemType: Sum + FromPrimitive;
}
