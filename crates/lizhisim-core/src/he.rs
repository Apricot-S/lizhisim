// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use heapless::Vec;
use thiserror::Error;

use crate::tile::TileKind;

const MAX_SIPAI_COUNT: usize = 27;

#[derive(Debug, Error, PartialEq)]
#[error("he cannot hold another sipai")]
pub struct HeFull;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sipai {
    pub tile_kind: TileKind,
    pub moqie: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct He {
    sipai: Vec<Sipai, MAX_SIPAI_COUNT>,
}

impl He {
    pub(crate) const fn new() -> Self {
        Self { sipai: Vec::new() }
    }

    pub fn last(&self) -> Option<&Sipai> {
        self.sipai.last()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Sipai> {
        self.sipai.iter()
    }

    pub(crate) fn with_appended(mut self, sipai: Sipai) -> Result<Self, HeFull> {
        self.sipai.push(sipai).map_err(|_| HeFull)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_exposes_appended_sipai() {
        let sipai = Sipai {
            tile_kind: TileKind::M1,
            moqie: false,
        };
        let he = He::new().with_appended(sipai.clone()).unwrap();

        assert_eq!(he.iter().collect::<std::vec::Vec<_>>(), vec![&sipai]);
    }
}
