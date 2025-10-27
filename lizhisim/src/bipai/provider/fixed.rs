// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use super::Bipai;
use super::BipaiProvider;
use thiserror::Error;

#[derive(Debug, Clone)]
pub(crate) struct FixedBipaiProvider {}

#[derive(Debug, Error)]
pub(crate) enum FixedBipaiProviderError<B: Bipai> {
    #[error("")]
    Empty,
    #[error(transparent)]
    Bipai(#[from] B::Error),
}

impl<B: Bipai> BipaiProvider<B> for FixedBipaiProvider {
    type Error = FixedBipaiProviderError<B>;

    fn provide_bipai(&mut self) -> Result<B, Self::Error> {
        unimplemented!()
    }
}
