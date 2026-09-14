// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HongBaopaiConfigField {
    M0Count,
    P0Count,
    S0Count,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HongBaopaiConfig {
    pub m0_count: u8,
    pub p0_count: u8,
    pub s0_count: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawRuleSpec {
    pub hong_baopai: HongBaopaiConfig,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleSpec {
    hong_baopai: HongBaopaiConfig,
}

#[derive(Debug, Error, PartialEq)]
pub enum RuleSpecError {
    #[error("{field:?} count {actual_count} exceeds maximum {max_count}")]
    HongBaopaiCountOutOfRange {
        field: HongBaopaiConfigField,
        actual_count: u8,
        max_count: u8,
    },
}

impl HongBaopaiConfig {
    const fn validate(&self) -> Result<(), RuleSpecError> {
        match validate_hong_baopai_count(HongBaopaiConfigField::M0Count, self.m0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match validate_hong_baopai_count(HongBaopaiConfigField::P0Count, self.p0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match validate_hong_baopai_count(HongBaopaiConfigField::S0Count, self.s0_count) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        Ok(())
    }
}

const fn validate_hong_baopai_count(
    field: HongBaopaiConfigField,
    actual_count: u8,
) -> Result<(), RuleSpecError> {
    if actual_count > 4 {
        return Err(RuleSpecError::HongBaopaiCountOutOfRange {
            field,
            actual_count,
            max_count: 4,
        });
    }
    Ok(())
}

impl TryFrom<RawRuleSpec> for RuleSpec {
    type Error = RuleSpecError;

    fn try_from(raw: RawRuleSpec) -> Result<Self, Self::Error> {
        raw.hong_baopai.validate()?;

        Ok(Self {
            hong_baopai: raw.hong_baopai,
        })
    }
}

impl RuleSpec {
    pub const fn hong_baopai(&self) -> &HongBaopaiConfig {
        &self.hong_baopai
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(m0_count: u8, p0_count: u8, s0_count: u8) -> RawRuleSpec {
        RawRuleSpec {
            hong_baopai: HongBaopaiConfig {
                m0_count,
                p0_count,
                s0_count,
            },
        }
    }

    #[test]
    fn rule_spec_accepts_each_m0_count_from_zero_through_four() {
        assert!(
            [0, 1, 2, 3, 4]
                .map(|count| RuleSpec::try_from(raw(count, 0, 0)).is_ok())
                .into_iter()
                .all(|accepted| accepted)
        );
    }

    #[test]
    fn rule_spec_accepts_each_p0_count_from_zero_through_four() {
        assert!(
            [0, 1, 2, 3, 4]
                .map(|count| RuleSpec::try_from(raw(0, count, 0)).is_ok())
                .into_iter()
                .all(|accepted| accepted)
        );
    }

    #[test]
    fn rule_spec_accepts_each_s0_count_from_zero_through_four() {
        assert!(
            [0, 1, 2, 3, 4]
                .map(|count| RuleSpec::try_from(raw(0, 0, count)).is_ok())
                .into_iter()
                .all(|accepted| accepted)
        );
    }

    #[test]
    fn rule_spec_rejects_m0_count_above_four() {
        assert_eq!(
            RuleSpec::try_from(raw(5, 0, 0)),
            Err(RuleSpecError::HongBaopaiCountOutOfRange {
                field: HongBaopaiConfigField::M0Count,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn rule_spec_rejects_p0_count_above_four() {
        assert_eq!(
            RuleSpec::try_from(raw(0, 5, 0)),
            Err(RuleSpecError::HongBaopaiCountOutOfRange {
                field: HongBaopaiConfigField::P0Count,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }

    #[test]
    fn rule_spec_rejects_s0_count_above_four() {
        assert_eq!(
            RuleSpec::try_from(raw(0, 0, 5)),
            Err(RuleSpecError::HongBaopaiCountOutOfRange {
                field: HongBaopaiConfigField::S0Count,
                actual_count: 5,
                max_count: 4,
            }),
        );
    }
}
