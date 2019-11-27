pub mod erosion;
pub mod generator;

use std::iter::Sum;

use crate::map::Map;

pub trait Modifier {
    fn modify<M>(&mut self, map: &mut M)
    where
        M: Map,
        M::ItemType: Sum + From<f32>;
}
