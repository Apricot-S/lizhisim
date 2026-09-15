// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use lizhisim_rules::{HongBaopaiConfig, RuleSet, ValidatedRuleSet};

use crate::tile_set::TileSet;

pub(crate) fn rules_with_origin(
    first_zimo_origin: lizhisim_rules::FirstZimoOrigin,
) -> ValidatedRuleSet {
    ValidatedRuleSet::try_from(RuleSet {
        first_zimo_origin,
        hong_baopai: HongBaopaiConfig {
            m0_count: 1,
            p0_count: 1,
            s0_count: 1,
        },
    })
    .unwrap()
}

pub(crate) fn red_three_four_player() -> TileSet {
    TileSet::try_from(&rules_with_origin(lizhisim_rules::FirstZimoOrigin::Qipai)).unwrap()
}
