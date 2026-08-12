// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use thiserror::Error;

use crate::bingpai::BingpaiError;
use crate::he::HeFull;
use crate::tile::TileKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dapai {
    Moqie(TileKind),
    Shouqie(TileKind),
}

#[derive(Debug, Error, PartialEq)]
pub enum DapaiError {
    #[error(transparent)]
    Bingpai(#[from] BingpaiError),
    #[error(transparent)]
    HeFull(#[from] HeFull),
}
