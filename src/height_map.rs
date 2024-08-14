#[cfg(test)]
mod tests;

use std::ops::BitAnd;

use num::{Integer, Num, NumCast, ToPrimitive};

#[derive(Debug)]
pub struct HeightMap {
    pub edge_size: usize,
    pub buffer: Vec<f32>,
}

impl HeightMap {
    pub fn with_edge_size(edge_size: usize) -> Self {
        let edge_size = previous_power_of_two(edge_size);
        Self {
            edge_size,
            buffer: vec![0.0; edge_size * edge_size],
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.edge_size, self.edge_size)
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.edge_size as isize && y >= 0 && y < self.edge_size as isize
    }

    pub fn at(&self, x: isize, y: isize) -> Option<f32> {
        if !self.in_bounds(x, y) {
            return None;
        };
        Some(self.buffer[x as usize + self.edge_size * y as usize])
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut f32 {
        &mut self.buffer[x + self.edge_size * y]
    }

    pub fn normalize(&mut self) {
        let (current_min, current_max) = self.min_max();
        let range = current_max - current_min;

        for height in &mut self.buffer {
            *height = (*height - current_min) / range;
        }
    }

    pub fn min_max(&self) -> (f32, f32) {
        (
            self.buffer
                .iter()
                .cloned()
                .fold(f32::INFINITY, f32::min),
            self.buffer
                .iter()
                .cloned()
                .fold(f32::NEG_INFINITY, f32::max),
        )
    }
}

pub fn to_buf<T>(height_map: &HeightMap, target_min: T, target_max: T) -> Vec<T>
where
    T: Num + std::cmp::PartialOrd + Copy + ToPrimitive + NumCast + std::fmt::Display,
{
    let (current_min, current_max) = height_map.min_max();
    let current_range = current_max - current_min;
    let target_range = target_max - target_min;

    height_map
        .buffer
        .iter()
        .map(|&height| {
            let normalized = (height - current_min) / current_range;
            let scaled = T::from(normalized * target_range.to_f32().unwrap()).unwrap();
            scaled + target_min
        })
        .collect()
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
