pub mod erosion;
pub mod generator;
pub mod randomizer;
pub mod slope;

use crate::map::Map;

pub trait Modifier {
    fn modify<M: Map>(&self, map: &mut M);
}
