// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use crate::tile::Tile;
use pyo3::prelude::*;

/// Position of the claimed tile in the melded sequence.
/// Used in [`FuluMianzi::Shunzi`](self::FuluMianzi::Shunzi).
#[pyclass(eq, eq_int, rename_all = "UPPERCASE")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimedTilePosition {
    /// The claimed tile is the lowest in the sequence.
    /// For example, claiming a 3 to form a sequence of 3-4-5.
    Low,
    /// The claimed tile is the middle in the sequence.
    /// For example, claiming a 4 to form a sequence of 3-4-5.
    Middle,
    /// The claimed tile is the highest in the sequence.
    /// For example, claiming a 5 to form a sequence of 3-4-5.
    High,
}

/// 副露面子: Meld.
///
/// # Examples
///
/// ```
/// # use hule::{ClaimedTilePosition, FuluMianzi};
/// // 4-56p (Chii 4p Low)
/// let shunzi = FuluMianzi::Shunzi(12, ClaimedTilePosition::Low);
///
/// // 1-11z (Pon 1z)
/// let kezi = FuluMianzi::Kezi(27);
///
/// // 7777s (Concealed Kan 7s)
/// let angangzi = FuluMianzi::Angangzi(24);
///
/// // 7-777s (Open Kan 7s)
/// let minggangzi = FuluMianzi::Minggangzi(24);
/// ```
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuluMianzi {
    /// 順子: Sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hule::{ClaimedTilePosition, FuluMianzi};
    /// // 1-23m (Chii 1m Low)
    /// let shunzi_low = FuluMianzi::Shunzi(0, ClaimedTilePosition::Low);
    ///
    /// // 2-13m (Chii 2m Middle)
    /// let shunzi_middle = FuluMianzi::Shunzi(1, ClaimedTilePosition::Middle);
    ///
    /// // 3-12m (Chii 3m High)
    /// let shunzi_high = FuluMianzi::Shunzi(2, ClaimedTilePosition::High);
    /// ```
    Shunzi(Tile, ClaimedTilePosition),
    /// 刻子: Triplet.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hule::FuluMianzi;
    /// // 1-11m (Pon 1m)
    /// let kezi = FuluMianzi::Kezi(0);
    /// ```
    Kezi(Tile),
    /// 暗槓子: Concealed Quad.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hule::FuluMianzi;
    /// // 1111m (Concealed Kan 1m)
    /// let angangzi = FuluMianzi::Angangzi(0);
    /// ```
    Angangzi(Tile),
    /// 明槓子: Open Quad.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hule::FuluMianzi;
    /// // 1-111m (Open Kan 1m)
    /// let minggangzi = FuluMianzi::Minggangzi(0);
    /// ```
    Minggangzi(Tile),
}
