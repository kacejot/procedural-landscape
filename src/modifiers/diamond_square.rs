use std::iter::Sum;

use num::{FromPrimitive, Num};
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    Rng,
};

use crate::{Map, Modifier};

type DiamondSquareRandomizer<R> = DistIter<Uniform<f32>, R, f32>;

pub struct DiamondSquare<R> {
    rng: DiamondSquareRandomizer<R>,
    with_flat_border: bool,
}

impl<R> DiamondSquare<R>
where
    R: Rng,
{
    pub fn new(rng: R, with_flat_border: bool) -> Self {
        let rng = Uniform::new(-1.0, 1.0).sample_iter(rng);
        Self {
            rng,
            with_flat_border,
        }
    }

    fn square_step<M>(&mut self, height_map: &mut M, step_size: usize, x: usize, y: usize)
    where
        M: Map,
        M::ItemType: Sum + FromPrimitive,
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
        M: Map,
        M::ItemType: Sum + FromPrimitive,
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
        T: Num + FromPrimitive,
    {
        base + T::from_f32(self.rng.next().unwrap() * amplitude as f32).unwrap()
    }
}

impl<R> Modifier for DiamondSquare<R>
where
    R: Rng,
{
    fn modify<M>(&mut self, map: &mut M)
    where
        M: Map,
        M::ItemType: Sum + FromPrimitive,
    {
        let mut step_size = map.edge_size();
        while step_size > 1 {
            let half = step_size / 2;

            let border = if self.with_flat_border {
                step_size
            } else {
                0
            };

            for x in (border..map.edge_size() - border).step_by(step_size) {
                for y in (border..map.edge_size() - border).step_by(step_size) {
                    self.square_step(map, step_size, x + half, y + half);
                    self.diamond_step(map, step_size, x + half, y);
                    self.diamond_step(map, step_size, x, y + half);
                }
            }
            step_size /= 2;
        }
    }
}
