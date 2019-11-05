use super::Map;
use crate::util::num::previous_power_of_two;
use num::Num;

pub struct Chunk<T> {
    pub edge_size: usize,
    pub height_map: Vec<T>,
}

impl<T> Chunk<T>
where
    T: Num + Copy,
{
    pub fn with_edge_size(edge_size: usize) -> Self {
        let edge_size = previous_power_of_two(edge_size);
        Self {
            edge_size,
            height_map: vec![T::zero(); edge_size * edge_size],
        }
    }
}

impl<T: Num + Copy> Map for Chunk<T> {
    type ItemType = T;

    fn edge_size(&self) -> usize {
        self.edge_size
    }

    fn at(&self, x: usize, y: usize) -> Self::ItemType {
        self.height_map[x + self.edge_size * y]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Self::ItemType {
        &mut self.height_map[x + self.edge_size * y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_is_2_and_len_is_4_if_create_with_edge_size_2() {
        let chunk = Chunk::<f32>::with_edge_size(2);
        assert_eq!(2, chunk.edge_size);
        assert_eq!(4, chunk.height_map.len());
    }

    #[test]
    fn edge_is_4_and_len_is_16_if_create_with_edge_size_5() {
        let chunk = Chunk::<f32>::with_edge_size(5);
        assert_eq!(4, chunk.edge_size);
        assert_eq!(16, chunk.height_map.len());
    }
}
