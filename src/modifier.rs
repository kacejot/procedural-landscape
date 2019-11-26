pub mod erosion;
pub mod generator;
pub mod slope;

use crate::map::Map;

pub trait Modifier {
    fn modify<M>(&mut self, map: &mut M)
    where
        M: Map,
        M::ItemType: From<f64>;
}
