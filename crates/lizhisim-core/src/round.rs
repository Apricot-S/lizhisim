// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

use core::marker::PhantomData;

use crate::bipai::{Bipai, PlayerSet, QipaiCompleted, QipaiPending};
use crate::seat::{FourPlayer, Seat};

pub struct Round<P: PlayerSet, State> {
    state: State,
    player_set: PhantomData<fn() -> P>,
}

pub struct Prepared<P: PlayerSet> {
    bipai: Bipai<P, QipaiPending>,
    zhuangjia: Seat<P>,
}

pub struct AwaitingDraw<P: PlayerSet> {
    #[expect(dead_code, reason = "will be consumed by the next zimo transition")]
    bipai: Bipai<P, QipaiCompleted>,
    #[expect(dead_code, reason = "will be consumed by player hand transitions")]
    bingpai: P::BingpaiSet,
    #[expect(dead_code, reason = "will identify the first zimo actor")]
    zhuangjia: Seat<P>,
}

impl Round<FourPlayer, Prepared<FourPlayer>> {
    pub fn new(bipai: Bipai<FourPlayer, QipaiPending>, zhuangjia: Seat<FourPlayer>) -> Self {
        Self {
            state: Prepared { bipai, zhuangjia },
            player_set: PhantomData,
        }
    }

    pub fn qipai(self) -> Round<FourPlayer, AwaitingDraw<FourPlayer>> {
        let (bipai, bingpai) = self.state.bipai.qipai();
        Round {
            state: AwaitingDraw {
                bipai,
                bingpai,
                zhuangjia: self.state.zhuangjia,
            },
            player_set: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bipai, FourPlayer, Seat, TileKind, TileSet};

    fn red_three_tiles() -> ([TileKind; 136], TileSet) {
        let tile_set = TileSet::red_three_four_player();

        let mut tiles = [TileKind::M1; 136];
        let mut cursor = 0;
        for tile_kind in TileKind::ALL {
            for _ in 0..tile_set.max_count(tile_kind) {
                tiles[cursor] = tile_kind;
                cursor += 1;
            }
        }
        (tiles, tile_set)
    }

    #[test]
    fn qipai_transitions_prepared_round_to_awaiting_draw() {
        let (tiles, tile_set) = red_three_tiles();
        let bipai = Bipai::<FourPlayer>::try_new(tiles, tile_set).unwrap();
        let prepared = Round::new(bipai, Seat::<FourPlayer>::ALL[0]);

        let actual = prepared.qipai();

        fn is_awaiting_draw(_: &Round<FourPlayer, AwaitingDraw<FourPlayer>>) -> bool {
            true
        }
        assert!(is_awaiting_draw(&actual));
    }
}
