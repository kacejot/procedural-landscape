use std::iter::Sum;

use num::{Float, FromPrimitive};
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    Rng,
};

use crate::{height_map::HeightMap, map::Grid};

type DiamondSquareRandomizer<R> = DistIter<Uniform<f32>, R, f32>;

pub struct DiamondSquare<R> {
    rng: DiamondSquareRandomizer<R>,
    smooth_borders: bool,
}

impl<R> DiamondSquare<R>
where
    R: Rng,
{
    pub fn new(rng: R, smooth_borders: bool) -> Self {
        let rng = Uniform::new(-1.0, 1.0).sample_iter(rng);
        Self {
            rng,
            smooth_borders,
        }
    }

    pub fn modify<M>(&mut self, map: &mut M)
    where
        M: Grid,
        M::ItemType: Float + Sum + Copy + FromPrimitive,
    {
        let (edge_size, _) = map.dimensions();
        let mut step_size = edge_size;
        while step_size > 1 {
            let half = step_size / 2;

            for x in (0..edge_size).step_by(step_size) {
                for y in (0..edge_size).step_by(step_size) {
                    self.square_step(map, step_size, x + half, y + half);

                    if !self.smooth_borders || y != 0 {
                        self.diamond_step(map, step_size, x + half, y);
                    }

                    if !self.smooth_borders || x != 0 {
                        self.diamond_step(map, step_size, x, y + half);
                    }
                }
            }
            step_size /= 2;
        }

        map.normalize();
    }

    fn square_step<M>(&mut self, height_map: &mut M, step_size: usize, x: usize, y: usize)
    where
        M: Grid,
        M::ItemType: Float + Sum + Copy + FromPrimitive,
    {
        let base = height_map
            .square_corners(x, y, step_size)
            .iter()
            .flatten()
            .copied()
            .sum::<M::ItemType>()
            / M::ItemType::from_f32(4.0).unwrap();
        *height_map.at_mut(x, y) = self.displace(base, step_size);
    }

    fn diamond_step<M>(&mut self, height_map: &mut M, step_size: usize, x: usize, y: usize)
    where
        M: Grid,
        M::ItemType: Float + Sum + Copy + FromPrimitive,
    {
        let base = height_map
            .diamond_corners(x, y, step_size)
            .iter()
            .flatten()
            .copied()
            .sum::<M::ItemType>()
            / M::ItemType::from_f32(4.0).unwrap();
        *height_map.at_mut(x, y) = self.displace(base, step_size);
    }

    fn displace<T>(&mut self, base: T, amplitude: usize) -> T
    where
        T: Float + FromPrimitive,
    {
        base + T::from_f32(self.rng.next().unwrap() * amplitude as f32).unwrap()
    }
}

pub fn generate<R: Rng>(rng: R, edge_size: usize) -> HeightMap<f32> {
    let mut map = HeightMap::<f32>::with_edge_size(edge_size);
    let mut ds = DiamondSquare::new(rng, true);

    ds.modify(&mut map);
    map
}
