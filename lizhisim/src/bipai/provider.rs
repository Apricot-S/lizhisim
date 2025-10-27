// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

//pub(crate) mod fixed;
pub(crate) mod random;

use super::bipai::Bipai;

pub(crate) trait BipaiProvider<B: Bipai>: Clone {
    type Error;

    fn provide_bipai(&mut self) -> Result<B, Self::Error>;
}
