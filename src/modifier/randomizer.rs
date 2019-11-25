use rand::{distributions::Distribution, Rng, SeedableRng};

pub struct Randomizer<R, D, T>
where
    R: SeedableRng + Rng,
    D: Distribution<T>,
{
    rng: R,
    distribution: D,
    phantom: ::core::marker::PhantomData<T>,
}

impl<R, D, T> Randomizer<R, D, T>
where
    R: SeedableRng + Rng,
    D: Distribution<T>,
{
    pub fn new(rng: R, distribution: D) -> Self {
        Self {
            rng,
            distribution,
            phantom: ::core::marker::PhantomData,
        }
    }

    pub fn with_seedable_rng<U>(rng: U) -> WithSeedableRng<U>
    where
        U: SeedableRng + Rng,
    {
        WithSeedableRng { rng }
    }

    pub fn next(&mut self) -> T {
        self.distribution.sample(&mut self.rng)
    }
}

pub struct WithSeedableRng<R>
where
    R: SeedableRng + Rng,
{
    rng: R,
}

impl<R> WithSeedableRng<R>
where
    R: SeedableRng + Rng,
{
    pub fn with_distribution<D, T>(self, distribution: D) -> WithDistribution<R, D, T>
    where
        D: Distribution<T>,
    {
        WithDistribution {
            rng: self.rng,
            distribution,
            phantom: ::core::marker::PhantomData,
        }
    }
}

pub struct WithDistribution<R, D, T>
where
    R: SeedableRng + Rng,
    D: Distribution<T>,
{
    rng: R,
    distribution: D,
    phantom: ::core::marker::PhantomData<T>,
}

impl<R, D, T> WithDistribution<R, D, T>
where
    R: SeedableRng + Rng,
    D: Distribution<T>,
{
    pub fn build(self) -> Randomizer<R, D, T> {
        Randomizer {
            rng: self.rng,
            distribution: self.distribution,
            phantom: ::core::marker::PhantomData,
        }
    }
}
