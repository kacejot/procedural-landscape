pub mod erosion;
pub mod generator;
pub mod slope;

use std::iter::Sum;

use crate::map::Map;

pub trait Modifier {
    fn modify<M>(&mut self, map: &mut M)
    where
        M: Map,
        M::ItemType: Sum + From<f32>;
}
