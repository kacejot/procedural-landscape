use rand::{
    distributions::{DistIter, Distribution, Uniform},
    rngs::StdRng,
    Rng, SeedableRng,
};

type DiamondSquareRandomizer<R> = DistIter<Uniform<f64>, R, f64>;

struct DiamondSquare<R> {
    rng: DiamondSquareRandomizer<R>,
    roughness: f32,
}

impl<R> DiamondSquare<R>
where
    R: Rng,
{
    fn new(rng: R, roughness: f32) -> Self {
        let rng = Uniform::new(-1.0, 1.0).sample_iter(rng);
        Self { rng, roughness }
    }
}
