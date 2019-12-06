use std::iter::Sum;

use num::Num;
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    Rng,
};

use crate::{map::Map, map_modifier::MapModifier};

type DiamondSquareRandomizer<R> = DistIter<Uniform<f32>, R, f32>;

pub struct DiamondSquare<R> {
    rng: DiamondSquareRandomizer<R>,
    roughness: f32,
}

impl<R> DiamondSquare<R>
where
    R: Rng,
{
    pub fn new(rng: R, roughness: f32) -> Self {
        let rng = Uniform::new(-1.0, 1.0).sample_iter(rng);
        Self { rng, roughness }
    }

    fn square_step<M>(&mut self, height_map: &mut M, step_size: usize, x: usize, y: usize)
    where
        M: Map,
        M::ItemType: Sum + From<f32>,
    {
        let base = height_map
            .square_corners(x, y, step_size)
            .iter()
            .flatten()
            .copied()
            .sum::<M::ItemType>()
            / M::ItemType::from(4.0);
        *height_map.at_mut(x, y) = self.displace(base, step_size);
    }

    fn diamond_step<M>(&mut self, height_map: &mut M, step_size: usize, x: usize, y: usize)
    where
        M: Map,
        M::ItemType: Sum + From<f32>,
    {
        let base = height_map
            .diamond_corners(x, y, step_size)
            .iter()
            .flatten()
            .copied()
            .sum::<M::ItemType>()
            / M::ItemType::from(4.0);
        *height_map.at_mut(x, y) = self.displace(base, step_size);
    }

    fn displace<T>(&mut self, base: T, amplitude: usize) -> T
    where
        T: Num + From<f32>,
    {
        let amplitude = amplitude as f32;
        base + T::from(self.rng.next().unwrap() * amplitude * self.roughness)
    }
}

impl<R> MapModifier for DiamondSquare<R>
where
    R: Rng,
{
    fn modify<M>(&mut self, map: &mut M)
    where
        M: Map,
        M::ItemType: Sum + From<f32>,
    {
        let mut step_size = map.edge_size();
        while step_size > 1 {
            let half = step_size / 2;
            for x in (0..map.edge_size()).step_by(step_size) {
                for y in (0..map.edge_size()).step_by(step_size) {
                    self.square_step(map, step_size, x + half, y + half);
                    self.diamond_step(map, step_size, x + half, y);
                    self.diamond_step(map, step_size, x, y + half);
                }
            }
            step_size /= 2;
        }
    }
}
