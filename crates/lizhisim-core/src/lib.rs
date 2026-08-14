// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/lizhisim

mod action;
mod bingpai;
mod bipai;
mod he;
mod player;
mod player_set;
mod round;
mod score;
mod seat;
mod table_match;
mod tile;
mod tile_set;

pub use action::{Dapai, DapaiError};
pub use bingpai::{Bingpai, BingpaiError};
pub use bipai::{Bipai, BipaiError, BipaiSpec, QipaiCompleted, QipaiPending};
pub use he::{He, HeFull, Sipai};
pub use player::Player;
pub use player_set::{FourPlayer, PlayerSet};
pub use round::{
    DapaiCompleted, FirstZimoOrigin, NoReactionResult, Round, RoundEnded, RoundOutcome,
    ZimoCompleted, ZimoPending,
};
pub use score::Score;
pub use seat::{Seat, SeatIndexOutOfRange};
pub use table_match::{Ben, Chang, Lizhibang, RoundIndex, RoundSettlement, TableMatchState};
pub use tile::TileKind;
pub use tile_set::{TileSet, TileSetError};
