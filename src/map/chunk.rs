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

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.edge_size as isize && y >= 0 && y < self.edge_size as isize
    }

    fn at(&self, x: isize, y: isize) -> Option<Self::ItemType> {
        if !self.in_bounds(x, y) {
            return None;
        };
        Some(self.height_map[x as usize + self.edge_size * y as usize])
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Self::ItemType {
        &mut self.height_map[x + self.edge_size * y]
    }

    fn square_corners(&self, x: isize, y: isize, edge_size: usize) -> Vec<Option<Self::ItemType>> {
        let half_edge = (edge_size / 2) as isize;
        (-half_edge..half_edge)
            .step_by(edge_size)
            .flat_map(|row| std::iter::repeat(row).zip((-half_edge..half_edge).step_by(edge_size)))
            .map(move |(row, col)| self.at(x + row, y + col))
            .collect()
    }

    fn diamond_corners(&self, x: isize, y: isize, diagonal: usize) -> Vec<Option<Self::ItemType>> {
        let half = (diagonal / 2) as isize;
        
        // gives -half, 0, half, 0, -half ...
        let sequence = (-half..=half)
            .step_by(half as usize)
            .chain(std::iter::once(0))
            .cycle();
        
        // gives (0, -half), (half, 0), (0, half), ...
        sequence.clone()
            .skip(1)
            .take(4)
            .zip(sequence.take(4))
            .map(|(row, col)| self.at(x + row, y + col))
            .collect()
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
