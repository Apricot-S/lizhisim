mod bingpai;
mod calculate;
mod constants;
mod mianzi;
mod qiduizi;
mod shisanyao;
mod shoupai;
mod standard;

pub use bingpai::Bingpai;
pub use calculate::{
    calculate_replacement_number, calculate_replacement_number_3_player,
    calculate_xiangting_number, calculate_xiangting_number_3_player, XiangtingError,
};
pub use mianzi::{ClaimedTilePosition, Mianzi};
