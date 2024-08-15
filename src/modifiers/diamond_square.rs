use num::{Float, FromPrimitive};
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    Rng,
};

use crate::data::HeightMap;

type DiamondSquareRandomizer<R> = DistIter<Uniform<f32>, R, f32>;

pub struct DiamondSquare<R> {
    rng: DiamondSquareRandomizer<R>,
}

impl<R> DiamondSquare<R>
where
    R: Rng,
{
    pub fn new(rng: R) -> Self {
        let rng = Uniform::new(-1.0, 1.0).sample_iter(rng);
        Self {
            rng,
        }
    }

    pub fn modify(&mut self, map: &mut HeightMap) {
        let mut step_size = map.chunk_size;
        while step_size > 1 {
            let half = step_size / 2;

            for x in (0..map.width).step_by(step_size as usize) {
                for y in (0..map.height).step_by(step_size as usize) {
                    self.square_step(map, step_size, x + half, y + half);
                    self.diamond_step(map, step_size, x + half, y);
                    self.diamond_step(map, step_size, x, y + half);
                }
            }
            step_size /= 2;
        }
        map.normalize();
    }

    fn square_step(&mut self, height_map: &mut HeightMap, step_size: u32, x: u32, y: u32) {
        let base = square_corners(&height_map, x, y, step_size)
            .iter()
            .flatten()
            .copied()
            .sum::<f32>()
            / 4.0;
        *height_map.at_mut(x, y) = self.displace(base, step_size);
    }

    fn diamond_step(&mut self, height_map: &mut HeightMap, step_size: u32, x: u32, y: u32) {
        let base = diamond_corners(height_map, x, y, step_size)
            .iter()
            .flatten()
            .copied()
            .sum::<f32>()
            / 4.0;
        *height_map.at_mut(x, y) = self.displace(base, step_size);
    }

    fn displace<T>(&mut self, base: T, amplitude: u32) -> T
    where
        T: Float + FromPrimitive,
    {
        base + T::from_f32(self.rng.next().unwrap() * amplitude as f32).unwrap()
    }
}

fn square_corners(height_map: &HeightMap, x: u32, y: u32, edge: u32) -> [Option<f32>; 4] {
    let x = x as i32;
    let y = y as i32;
    let half_edge = (edge / 2) as i32;

    [
        height_map.at(x - half_edge, y - half_edge),
        height_map.at(x + half_edge, y - half_edge),
        height_map.at(x + half_edge, y + half_edge),
        height_map.at(x - half_edge, y + half_edge),
    ]
}

fn diamond_corners(
    height_map: &HeightMap,
    x: u32,
    y: u32,
    diagonal: u32,
) -> [Option<f32>; 4] {
    let x = x as i32;
    let y = y as i32;
    let half_diagonal = (diagonal / 2) as i32;

    [
        height_map.at(x, y - half_diagonal),
        height_map.at(x + half_diagonal, y),
        height_map.at(x, y + half_diagonal),
        height_map.at(x - half_diagonal, y),
    ]
}
