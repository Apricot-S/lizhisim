// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use super::Bipai;
use super::BipaiProvider;
use rand::{Rng, SeedableRng};
use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RandomBipaiProvider<B, R>
where
    B: Bipai,
    R: Rng + SeedableRng + Clone,
    B::Config: Clone,
{
    rng: R,
    config: B::Config,
}

impl<B, R> RandomBipaiProvider<B, R>
where
    B: Bipai,
    R: Rng + SeedableRng + Clone,
    B::Config: Clone,
{
    pub(crate) const fn new(rng: R, config: B::Config) -> Self {
        Self { rng, config }
    }
}

impl<B, R> Clone for RandomBipaiProvider<B, R>
where
    B: Bipai,
    R: Rng + SeedableRng + Clone,
    B::Config: Clone,
{
    fn clone(&self) -> Self {
        Self {
            rng: self.rng.clone(),
            config: self.config.clone(),
        }
    }
}

impl<B, R> BipaiProvider<B> for RandomBipaiProvider<B, R>
where
    B: Bipai,
    R: Rng + SeedableRng + Clone,
    B::Config: Clone,
{
    type Error = Infallible;

    fn provide_bipai(&mut self) -> Result<B, Self::Error> {
        Ok(Bipai::new(&mut self.rng, &self.config))
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::bipai::four_player::{Bipai4p, Bipai4pConfig, HongbaopaiCount};
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn provide_bipai_returns_new_instance() {
        let rng = StdRng::seed_from_u64(42);
        let config = Bipai4pConfig {
            hongbaopai_count: HongbaopaiCount::new(1, 1, 1).unwrap(),
        };
        let mut provider = RandomBipaiProvider::<Bipai4p, _>::new(rng, config);

        provider.provide_bipai().unwrap();
    }

    #[test]
    fn cloned_provide_bipai_returns_same_bipai() {
        let rng = StdRng::seed_from_u64(42);
        let config = Bipai4pConfig {
            hongbaopai_count: HongbaopaiCount::new(1, 1, 1).unwrap(),
        };
        let mut provider1 = RandomBipaiProvider::<Bipai4p, _>::new(rng, config);
        let mut provider2 = provider1.clone();

        let bipai1 = provider1.provide_bipai().unwrap();
        let bipai2 = provider2.provide_bipai().unwrap();
        assert_eq!(bipai1, bipai2);
    }
}
