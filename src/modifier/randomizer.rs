use rand::{distributions::Distribution, Rng, SeedableRng};

pub struct Randomizer<R, D> {
    rng: R,
    distribution: D,
}

impl<R, D, T> Randomizer<R, D>
where
    D: Distribution<T>,
    R: SeedableRng + Rng,
{
    pub fn with_seedable_rng(rng: R) -> WithSeedableRng<R> {
        WithSeedableRng { rng }
    }

    pub fn next(&mut self) -> T {
        self.distribution.sample(&mut self.rng)
    }
}

pub struct WithSeedableRng<R> {
    rng: R,
}

impl<R> WithSeedableRng<R>
where
    R: SeedableRng + Rng,
{
    pub fn with_distribution<D, T>(self, distribution: D) -> WithDistribution<R, D>
    where
        D: Distribution<T>,
    {
        WithDistribution {
            rng: self.rng,
            distribution,
        }
    }
}

pub struct WithDistribution<R, D> {
    rng: R,
    distribution: D,
}

impl<R, D, T> WithDistribution<R, D>
where
    R: SeedableRng + Rng,
    D: Distribution<T>,
{
    pub fn build(self) -> Randomizer<R, D> {
        Randomizer {
            rng: self.rng,
            distribution: self.distribution,
        }
    }
}
