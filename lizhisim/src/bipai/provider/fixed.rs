// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use super::Bipai;
use super::BipaiProvider;
use std::marker::PhantomData;

pub(crate) struct FixedBipaiProvider<B: Bipai> {
    _marker: PhantomData<B>,
}

impl<B: Bipai> BipaiProvider<B> for FixedBipaiProvider<B> {
    type Error = B::Error;

    fn provide_bipai(&mut self) -> Result<B, Self::Error> {
        unimplemented!()
    }
}
