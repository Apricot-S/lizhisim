// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use super::Bipai;
use super::BipaiProvider;
use std::collections::VecDeque;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FixedBipaiProvider<B>
where
    B: Bipai,
    B::Config: Clone,
{
    bipai_list: VecDeque<Vec<u8>>,
    config: B::Config,
}

#[derive(Debug, Error)]
pub(crate) enum FixedBipaiProviderError<B: Bipai> {
    #[error("no bipai left to provide")]
    Empty,
    #[error(transparent)]
    Bipai(B::Error),
}

impl<B> FixedBipaiProvider<B>
where
    B: Bipai,
    B::Config: Clone,
{
    pub(crate) fn new<I>(bipai_list: I, config: B::Config) -> Self
    where
        I: IntoIterator<Item = Vec<u8>>,
    {
        Self {
            bipai_list: bipai_list.into_iter().collect(),
            config,
        }
    }
}

impl<B> Clone for FixedBipaiProvider<B>
where
    B: Bipai,
    B::Config: Clone,
{
    fn clone(&self) -> Self {
        Self {
            bipai_list: self.bipai_list.clone(),
            config: self.config.clone(),
        }
    }
}

impl<B> BipaiProvider<B> for FixedBipaiProvider<B>
where
    B: Bipai,
    B::Config: Clone,
{
    type Error = FixedBipaiProviderError<B>;

    fn provide_bipai(&mut self) -> Result<B, Self::Error> {
        let Some(bipai) = self.bipai_list.pop_front() else {
            return Err(FixedBipaiProviderError::Empty);
        };

        Bipai::from_slice(&bipai, &self.config).map_err(|e| FixedBipaiProviderError::Bipai(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
