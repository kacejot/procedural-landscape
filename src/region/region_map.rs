use super::{area::Area, region_neighbourship::RegionNeighbour};
use num::Zero;

pub struct RegionMap<'a, T> {
    pub edge_size: usize,
    pub height_map: Vec<T>,
    pub neighbouring_regions: [RegionNeighbour<'a, T>; 8],
}

impl<'a, T> RegionMap<'a, T>
where
    Self: Area,
    T: Zero + Clone,
    RegionNeighbour<'a, T>: Copy,
{
    pub fn with_edge_size(mut edge_size: usize) -> Self {
        edge_size = (edge_size / 2).next_power_of_two();
        Self {
            edge_size,
            height_map: vec![T::zero(); edge_size * edge_size],
            neighbouring_regions: [RegionNeighbour::None; 8],
        }
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.height_map[x + y * self.edge_size]
    }
}

impl<'a, T> Area for RegionMap<'a, T>
where
    T: Copy,
{
    type ItemType = T;
    fn at(&self, x: usize, y: usize) -> Self::ItemType {
        self.height_map[x + y * self.edge_size]
    }
}
