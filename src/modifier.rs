pub mod erosion;
pub mod generator;
pub mod slope;

use crate::map::Map;
use num::Num;

pub trait Modifier {
    type MapType: Map;
    fn modify(&self, map: &mut Self::MapType);
}
