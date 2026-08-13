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
    #[error("actor index {actor_index} is out of range for four players")]
    ActorIndexOutOfRange { actor_index: usize },
    #[error("moqie is unavailable for the initial-deal dealer first dapai")]
    MoqieUnavailableForInitialDealFirstDapai,
    #[error(transparent)]
    Bingpai(#[from] BingpaiError),
    #[error(transparent)]
    HeFull(#[from] HeFull),
}
