use rand::{distributions::Uniform, rngs::StdRng, SeedableRng, Rng};

use crate::modifier::randomizer::Randomizer;

struct DiamondSquare
{
    rng: Randomizer<StdRng, Uniform<f64>, f64>,
    roughness: f32,
}

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
