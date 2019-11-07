pub mod erosion;
pub mod generator;
pub mod slope;

use crate::map::Map;
use num::Num;

pub trait Modifier {
    fn modify<T: Map>(&self, map: &mut T)
    where
        T: Map,
        T::ItemType: Num;
}
