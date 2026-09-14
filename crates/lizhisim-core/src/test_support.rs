// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use lizhisim_rules::{HongBaopaiConfig, RawRuleSpec, RuleSpec};

use crate::tile_set::TileSet;

pub(crate) fn red_three_four_player() -> TileSet {
    let rules = RuleSpec::try_from(RawRuleSpec {
        hong_baopai: HongBaopaiConfig {
            m0_count: 1,
            p0_count: 1,
            s0_count: 1,
        },
    })
    .unwrap();
    TileSet::try_from(&rules).unwrap()
}
