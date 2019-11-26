use num::Num;
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    Rng,
};

use crate::{map::Map, modifier::Modifier};

type DiamondSquareRandomizer<R> = DistIter<Uniform<f64>, R, f64>;

struct DiamondSquare<R> {
    rng: DiamondSquareRandomizer<R>,
    roughness: f64,
}

impl<R> DiamondSquare<R>
where
    R: Rng,
{
    fn new(rng: R, roughness: f64) -> Self {
        let rng = Uniform::new(-1.0, 1.0).sample_iter(rng);
        Self { rng, roughness }
    }

    fn square_step<M>(&mut self, height_map: &mut M, step_size: usize)
    where
        M: Map,
        M::ItemType: Num + From<f64>,
    {
        (0..height_map.edge_size())
            .step_by(step_size)
            .flat_map(|row| std::iter::repeat(row).zip(0..height_map.edge_size()))
            .for_each(|(x, y)| {
                let _corners = height_map.square_corners(x, y, step_size);
                // TODO: check for corners presence (None, Some)
            });
    }

    fn diamond_step<M>(&mut self, _height_map: &mut M, _step_size: usize)
    where
        M: Map,
        M::ItemType: From<f64>,
    {
    }

    fn displace<T>(&mut self, base: T, amplitude: usize) -> T
    where
        T: Num + From<f64>,
    {
        let amplitude = amplitude as f64;
        base + T::from(self.rng.next().unwrap() * amplitude * self.roughness)
    }
}

impl<R> Modifier for DiamondSquare<R>
where
    R: Rng,
{
    fn modify<M>(&mut self, map: &mut M)
    where
        M: Map,
        M::ItemType: From<f64>,
    {
        let mut step_size = map.edge_size();
        while step_size > 1 {
            self.square_step(map, step_size);
            self.diamond_step(map, step_size);
            step_size /= 2;
        }
    }
}
