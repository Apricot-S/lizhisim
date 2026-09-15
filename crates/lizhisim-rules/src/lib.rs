// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

//! Rule configuration and preset resolution for LizhiSim.

mod rule_set;

pub use rule_set::{
    HongBaopaiConfig, HongBaopaiConfigField, RuleSet, RuleSetValidationError, ValidatedRuleSet,
};
