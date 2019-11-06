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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Chunk;

    struct MockModifier;
    impl Modifier for MockModifier {
        fn modify<T>(&self, map: &mut T)
        where
            T: Map,
            T::ItemType: Num,
        {
            *map.at_mut(0, 0) = T::ItemType::one();
        }
    }

    #[test]
    fn modify_map() {
        let modifier = MockModifier {};
        let mut map = Chunk::<f32>::with_edge_size(1);
        assert_eq!(map.at(0, 0), 0.0);
        modifier.modify(&mut map);
        assert_eq!(map.at(0, 0), 1.0);
    }
}
