// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use heapless::Vec;

use crate::tile::TileKind;

const MAX_SIPAI_COUNT: usize = 27;

pub struct Sipai {
    pub tile_kind: TileKind,
    pub moqie: bool,
}

pub struct He {
    sipai: Vec<Sipai, MAX_SIPAI_COUNT>,
}

impl He {
    pub(crate) const fn new() -> Self {
        Self { sipai: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.sipai.is_empty()
    }
}
