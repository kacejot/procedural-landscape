#[cfg(test)]
mod tests;

use num::Num;

use super::Map;
use crate::util::num::previous_power_of_two;

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

    fn square_corners(&self, x: usize, y: usize, edge: usize) -> [Option<Self::ItemType>; 4] {
        let x = x as isize;
        let y = y as isize;
        let half_edge = (edge / 2) as isize;

        [
            self.at(x - half_edge, y - half_edge),
            self.at(x + half_edge, y - half_edge),
            self.at(x + half_edge, y + half_edge),
            self.at(x - half_edge, y + half_edge),
        ]
    }

    fn diamond_corners(&self, x: usize, y: usize, diagonal: usize) -> [Option<Self::ItemType>; 4] {
        let x = x as isize;
        let y = y as isize;
        let half_diagonal = (diagonal / 2) as isize;

        [
            self.at(x, y - half_diagonal),
            self.at(x + half_diagonal, y),
            self.at(x, y + half_diagonal),
            self.at(x - half_diagonal, y),
        ]
    }

    fn eight_neighbours(&self, x: usize, y: usize, edge: usize) -> [Option<Self::ItemType>; 8] {
        let corners = self.square_corners(x, y, edge);
        let centers = self.diamond_corners(x, y, edge);

        let mut result = [None; 8];
        let mut elements = result.iter_mut();
        for i in 0..4 {
            *elements.next().unwrap() = corners[i];
            *elements.next().unwrap() = centers[i];
        }

        result
    }
}
