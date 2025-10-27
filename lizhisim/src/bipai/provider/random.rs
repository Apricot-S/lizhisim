// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use super::super::bipai::four_player::Bipai4p;
use super::Bipai;
use super::BipaiProvider;
use rand::SeedableRng;
use std::convert::Infallible;
use std::marker::PhantomData;

pub(crate) struct RandomBipaiProvider<B: Bipai> {
    _marker: PhantomData<B>,
}

impl<B: Bipai> BipaiProvider<B> for RandomBipaiProvider<B> {
    type Error = Infallible;

    fn provide_bipai(&mut self) -> Result<B, Self::Error> {
        unimplemented!()
    }
}
