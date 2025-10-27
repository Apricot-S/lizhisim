// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use super::Bipai;
use super::BipaiProvider;
use rand::{Rng, SeedableRng};
use std::convert::Infallible;

#[derive(Debug)]
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
    fn new(rng: R, config: B::Config) -> Self {
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
