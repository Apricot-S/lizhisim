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
    #[error("no wall left to provide")]
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
    use super::super::super::bipai::four_player::{Bipai4p, Bipai4pConfig, HongbaopaiCount};
    use super::*;

    fn get_bipai_list_for_test() -> Vec<Vec<u8>> {
        let tiles1 = (0..136).map(|t| t / 4).collect::<Vec<u8>>();
        let tiles2 = (0..136).rev().map(|t| t / 4).collect::<Vec<u8>>();
        vec![tiles1, tiles2]
    }

    #[test]
    fn provide_bipai_returns_new_instance() {
        let bipai_list = get_bipai_list_for_test();
        let config = Bipai4pConfig {
            hongbaopai_count: HongbaopaiCount::new(0, 0, 0).unwrap(),
        };
        let mut provider = FixedBipaiProvider::<Bipai4p>::new(bipai_list, config);

        provider.provide_bipai().unwrap();
    }

    #[test]
    fn cloned_provide_bipai_returns_same_bipai() {
        let bipai_list = get_bipai_list_for_test();
        let config = Bipai4pConfig {
            hongbaopai_count: HongbaopaiCount::new(0, 0, 0).unwrap(),
        };
        let mut provider1 = FixedBipaiProvider::<Bipai4p>::new(bipai_list, config);
        let mut provider2 = provider1.clone();

        let bipai1 = provider1.provide_bipai().unwrap();
        let bipai2 = provider2.provide_bipai().unwrap();
        assert_eq!(bipai1, bipai2);
    }

    #[test]
    fn provide_bipai_returns_error_when_empty() {
        let bipai_list = get_bipai_list_for_test();
        let config = Bipai4pConfig {
            hongbaopai_count: HongbaopaiCount::new(0, 0, 0).unwrap(),
        };
        let mut provider = FixedBipaiProvider::<Bipai4p>::new(bipai_list, config);

        provider.provide_bipai().unwrap();
        provider.provide_bipai().unwrap();

        assert!(matches!(
            provider.provide_bipai().unwrap_err(),
            FixedBipaiProviderError::Empty
        ));
    }
}
