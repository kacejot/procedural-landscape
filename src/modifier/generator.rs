use rand::{distributions::Uniform, rngs::StdRng, SeedableRng, Rng};

use crate::modifier::randomizer::Randomizer;

struct DiamondSquare
{
    rng: Randomizer<StdRng, Uniform<f64>, f64>,
    roughness: f32,
}

// impl<R, D> DiamondSquare<R, D>
// where
//     R: SeedableRng + Rng,
//     D: Distribution<f64>,
// {
//     fn random<T: Num>(&mut self, n: T) -> T {
//         T::from(self.distribution.sample(&mut self.rng))
//     }
// }

impl DiamondSquare {
    fn new(seed: <StdRng as SeedableRng>::Seed, roughness: f32) -> Self {
        let rng = Randomizer::with_seedable_rng(StdRng::from_seed(seed))
            .with_distribution(Uniform::new(-1.0, 1.0))
            .build();

        Self {
            rng,
            roughness,
        }
    }
}

//     //    fn displace<T: Num>(&self, amplitude: usize, base: T) -> T
//     //    where rand::distributions::Standard: rand::distributions::Distribution<T> {
//     //        base + (self.generator.gen::<T>() - 0.5) * amplitude * self.roughness
//     //    }

//     fn square_step(&self, height_map: &mut T, step_size: usize) {
//         (0..height_map.edge_size())
//             .step_by(step_size)
//             .flat_map(|row| std::iter::repeat(row).zip(0..height_map.edge_size()))
//             .map(|(row, column)| {
//                 let corners = height_map.square_corners(row, column, step_size);
//             });
//     }

//     fn diamond_step(&self, height_map: &mut T, step_size: usize) {
//         let half_step = step_size / 2;
//         for i in (0..height_map.edge_size()).step_by(step_size) {
//             for j in (0..height_map.edge_size()).step_by(step_size) {}
//         }
//     }
// }

// impl Modifier<T> for DiamondSquare<T> {
//     type MapType = T;
//     fn modify(&self, map: &mut Self::MapType)
//     {
//         let mut step_size = map.edge_size();
//         while step_size > 1 {
//             self.square_step(map, step_size);
//             self.diamond_step(map, step_size);
//             step_size /= 2;
//         }
//     }
// }
