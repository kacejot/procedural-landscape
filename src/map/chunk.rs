#[cfg(test)]
mod tests;

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

    fn square_corners(&self, x: usize, y: usize, edge: usize) -> Vec<Option<Self::ItemType>> {
        let half_edge = (edge / 2) as isize;

        let sequence = [-half_edge, -half_edge, half_edge, half_edge];
        let sequence = sequence.iter().cycle();

        sequence
            .clone()
            .skip(1)
            .take(4)
            .zip(sequence.take(4))
            .map(|(row, col)| self.at(x as isize + row, y as isize + col))
            .collect()
    }

    fn diamond_corners(&self, x: usize, y: usize, diagonal: usize) -> Vec<Option<Self::ItemType>> {
        let half_diagonal = (diagonal / 2) as isize;

        let sequence = [-half_diagonal, 0, half_diagonal, 0];
        let sequence = sequence.iter().cycle();

        sequence
            .clone()
            .skip(1)
            .take(4)
            .zip(sequence.take(4))
            .map(|(row, col)| self.at(x as isize + row, y as isize + col))
            .collect()
    }

    fn eight_neighbours(&self, x: usize, y: usize, edge: usize) -> Vec<Option<Self::ItemType>> {
        let corners = self.square_corners(x, y, edge);
        let centers = self.diamond_corners(x, y, edge);

        let mut result = Vec::with_capacity(8);
        corners
            .iter()
            .zip(centers.iter())
            .for_each(|(corner, center)| {
                result.push(*corner);
                result.push(*center)
            });

        result
    }
}
