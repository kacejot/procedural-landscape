use rand::{SeedableRng, rngs::StdRng, Rng, distributions::{Distribution, Uniform}};

use num::Num;

use super::Modifier;
use crate::map::Map;

type Seed = [u8; 32];

struct DiamondSquare {
    seed: Seed,
    generator: StdRng,
    distribution: Uniform,
    roughness: f32,
}

impl DiamondSquare {
    fn new(seed: &Seed, roughness: f32) -> Self {
        Self {
            seed: *seed,
            generator: StdRng::from_seed(*seed),
            roughness
        }
    }

    fn displace<T: Num>(&self, amplitude: usize, base: T) -> T
    where rand::distributions::Standard: rand::distributions::Distribution<T> {
        base + (self.generator.gen::<T>() - 0.5) * amplitude * self.roughness
    }

    fn square_step<T: Map>(&self, height_map: &mut T, step_size: usize) {
        (0..height_map.edge_size())
            .step_by(step_size)
            .flat_map(|row| std::iter::repeat(row).zip(0..height_map.edge_size()))
            .map(|(row, column)| {
                let corners = height_map.square_corners(row, column, step_size);

            });
    }

    fn diamond_step<T: Map>(&self, height_map: &mut T, step_size: usize) {
        let half_step = step_size / 2;
        for i in (0..height_map.edge_size()).step_by(step_size) {
            for j in (0..height_map.edge_size()).step_by(step_size) {}
        }
    }
}


impl Modifier for DiamondSquare {
    fn modify<M>(&self, map: &mut M)
    where
        M: Map,
        M::ItemType: Num,
    {
        let mut step_size = map.edge_size();
        while step_size > 1 {
            self.square_step(map, step_size);
            self.diamond_step(map, step_size);
            step_size /= 2;
        }
    }
}
