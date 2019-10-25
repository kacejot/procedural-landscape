use super::{area::Area, region_map::RegionMap};
use num::Zero;

pub enum Neighbourship {
    NorthWest = 0,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

#[derive(Copy, Clone)]
pub enum RegionNeighbour<'a, T> {
    Region(&'a RegionMap<'a, T>),
    Flat(T),
    None,
}

impl<'a, T> Area for RegionNeighbour<'a, T>
where
    T: Zero + Copy,
{
    type ItemType = T;
    fn at(&self, x: usize, y: usize) -> Self::ItemType {
        match self {
            RegionNeighbour::Region(region) => region.at(x, y),
            RegionNeighbour::Flat(height) => *height,
            RegionNeighbour::None => T::zero(),
        }
    }
}
