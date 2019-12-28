#[cfg(test)]
mod tests;

use std::ops::BitAnd;

use num::{Float, Integer, Num, NumCast, ToPrimitive};

use crate::map::Grid;

#[derive(Debug)]
pub struct HeightMap<T> {
    pub edge_size: usize,
    pub buffer: Vec<T>,
}

impl<T> Grid for HeightMap<T>
where
    T: Num + Copy + std::cmp::PartialOrd + ToPrimitive + NumCast,
{
    type ItemType = T;

    fn dimensions(&self) -> (usize, usize) {
        (self.edge_size, self.edge_size)
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.edge_size as isize && y >= 0 && y < self.edge_size as isize
    }

    fn at(&self, x: isize, y: isize) -> Option<T> {
        if !self.in_bounds(x, y) {
            return None;
        };
        Some(self.buffer[x as usize + self.edge_size * y as usize])
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.buffer[x + self.edge_size * y]
    }

    fn square_corners(&self, x: usize, y: usize, edge: usize) -> [Option<T>; 4] {
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

    fn diamond_corners(&self, x: usize, y: usize, diagonal: usize) -> [Option<T>; 4] {
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

    fn eight_neighbours(&self, x: usize, y: usize, edge: usize) -> [Option<T>; 8] {
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

    fn normalize(&mut self) {
        let (current_min, current_max) = self.min_max();
        let range = current_max - current_min;

        for height in &mut self.buffer {
            *height = (*height - current_min) / range;
        }
    }
}

impl<T> HeightMap<T>
where
    T: ToPrimitive + std::cmp::PartialOrd + Num + Copy + NumCast,
{
    pub fn with_edge_size(edge_size: usize) -> Self {
        let edge_size = previous_power_of_two(edge_size);
        Self {
            edge_size,
            buffer: vec![T::zero(); edge_size * edge_size],
        }
    }

    pub fn edge_size(&self) -> usize {
        self.edge_size
    }

    pub fn scale<U>(&self, target_min: U, target_max: U) -> HeightMap<U>
    where
        U: Num + std::cmp::PartialOrd + Copy + ToPrimitive + NumCast + std::fmt::Display
    {
        let mut map = HeightMap::<U>::with_edge_size(self.edge_size());

        let (current_min, current_max) = self.min_max();

        let current_range = current_max - current_min;
        let target_range = target_max - target_min;

        for (i, height) in self.buffer.iter().enumerate() {
            let value = target_min
            + U::from((*height - current_min) / current_range * T::from(target_range).unwrap()).unwrap();
            map.buffer[i] = value;
        }

        map
    }

    fn min_max(&self) -> (T, T) {
        (
            *self
                .buffer
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap(),
            *self
                .buffer
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap(),
        )
    }
}

fn previous_power_of_two<T>(mut n: T) -> T
where
    T: Integer + Copy + BitAnd<Output = T>,
{
    loop {
        let temp = n & (n - T::one());
        if T::zero() == temp {
            break;
        }
        n = temp;
    }
    n
}
