mod an_gangzi;
mod daming_gangzi;
mod jia_gangzi;
mod mianzi;
mod ming_kezi;
mod ming_mianzi;
mod ming_shunzi;
mod tajia;

pub use an_gangzi::AnGangzi;
pub use daming_gangzi::DamingGangzi;
pub use jia_gangzi::JiaGangzi;
pub use mianzi::Mianzi;
pub use ming_kezi::MingKezi;
pub use ming_mianzi::MingMianzi;
pub use ming_shunzi::{ClaimedTilePosition, MingShunzi};
pub use tajia::Tajia;

#[derive(Clone)]
pub enum ChiPengGangMianzi {
    MingShunzi(MingShunzi),
    MingKezi(MingKezi),
    DamingGangzi(DamingGangzi),
}

#[derive(Clone)]
pub enum AnGangJiaGangMianzi {
    AnGangzi(AnGangzi),
    JiaGangzi(JiaGangzi),
}

#[derive(Clone)]
pub enum FuluMianzi {
    MingShunzi(MingShunzi),
    MingKezi(MingKezi),
    DamingGangzi(DamingGangzi),
    JiaGangzi(JiaGangzi),
    AnGangzi(AnGangzi),
}
