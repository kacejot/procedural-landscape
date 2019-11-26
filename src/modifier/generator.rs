use crate::map::Map;
use crate::modifier::Modifier;
use num::Num;
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    rngs::StdRng,
    Rng, SeedableRng,
};

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

    fn square_step<M>(&self, height_map: &mut M, step_size: usize)
    where
        M: Map,
    {
        (0..height_map.edge_size())
            .step_by(step_size)
            .flat_map(|row| std::iter::repeat(row).zip(0..height_map.edge_size()))
            .map(|(row, column)| {
                let corners = height_map.square_corners(row, column, step_size);
            });
    }

    fn diamond_step<M>(&self, height_map: &mut M, step_size: usize)
    where
        M: Map,
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
    fn modify<M>(&self, map: &mut M)
    where
        M: Map,
    {
        let mut step_size = map.edge_size();
        while step_size > 1 {
            self.square_step(map, step_size);
            self.diamond_step(map, step_size);
            step_size /= 2;
        }
    }
}
