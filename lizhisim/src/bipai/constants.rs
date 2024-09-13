use crate::tile;
use crate::tile::Tile;
use crate::tile::NUM_SAME_TILE;

pub(super) const NUM_BIPAI_4: usize = 136;
pub(super) const NUM_BIPAI_3: usize = 108;
pub(super) const NUM_WANGPAI: usize = 14;
pub(super) const NUM_LINGSHANGPAI: usize = 4;
pub(super) const NUM_LINGSHANGPAI_WITH_BABEI: usize = 8;
pub(super) const MAX_GANG_COUNT: usize = 4;
pub(super) const MAX_NUM_BAOPAI: usize = MAX_GANG_COUNT + 1;

#[rustfmt::skip]
pub(super) const INITIAL_BIPAI_WITHOUT_HONGBAOPAI_4: [Tile; NUM_BIPAI_4] = [
    tile!(1m), tile!(1m), tile!(1m), tile!(1m),
    tile!(2m), tile!(2m), tile!(2m), tile!(2m),
    tile!(3m), tile!(3m), tile!(3m), tile!(3m),
    tile!(4m), tile!(4m), tile!(4m), tile!(4m),
    tile!(5m), tile!(5m), tile!(5m), tile!(5m),
    tile!(6m), tile!(6m), tile!(6m), tile!(6m),
    tile!(7m), tile!(7m), tile!(7m), tile!(7m),
    tile!(8m), tile!(8m), tile!(8m), tile!(8m),
    tile!(9m), tile!(9m), tile!(9m), tile!(9m),

    tile!(1p), tile!(1p), tile!(1p), tile!(1p),
    tile!(2p), tile!(2p), tile!(2p), tile!(2p),
    tile!(3p), tile!(3p), tile!(3p), tile!(3p),
    tile!(4p), tile!(4p), tile!(4p), tile!(4p),
    tile!(5p), tile!(5p), tile!(5p), tile!(5p),
    tile!(6p), tile!(6p), tile!(6p), tile!(6p),
    tile!(7p), tile!(7p), tile!(7p), tile!(7p),
    tile!(8p), tile!(8p), tile!(8p), tile!(8p),
    tile!(9p), tile!(9p), tile!(9p), tile!(9p),

    tile!(1s), tile!(1s), tile!(1s), tile!(1s),
    tile!(2s), tile!(2s), tile!(2s), tile!(2s),
    tile!(3s), tile!(3s), tile!(3s), tile!(3s),
    tile!(4s), tile!(4s), tile!(4s), tile!(4s),
    tile!(5s), tile!(5s), tile!(5s), tile!(5s),
    tile!(6s), tile!(6s), tile!(6s), tile!(6s),
    tile!(7s), tile!(7s), tile!(7s), tile!(7s),
    tile!(8s), tile!(8s), tile!(8s), tile!(8s),
    tile!(9s), tile!(9s), tile!(9s), tile!(9s),

    tile!(1z), tile!(1z), tile!(1z), tile!(1z), // dong
    tile!(2z), tile!(2z), tile!(2z), tile!(2z), // nan
    tile!(3z), tile!(3z), tile!(3z), tile!(3z), // xi
    tile!(4z), tile!(4z), tile!(4z), tile!(4z), // bei
    tile!(5z), tile!(5z), tile!(5z), tile!(5z), // baiban
    tile!(6z), tile!(6z), tile!(6z), tile!(6z), // lufa
    tile!(7z), tile!(7z), tile!(7z), tile!(7z), // hongzhong
];

pub(super) const BIPAI_RED_5M_INDEX_4: usize = 4 * NUM_SAME_TILE as usize;
pub(super) const BIPAI_RED_5P_INDEX_4: usize = (9 + 4) * NUM_SAME_TILE as usize;
pub(super) const BIPAI_RED_5S_INDEX_4: usize = (9 * 2 + 4) * NUM_SAME_TILE as usize;

#[rustfmt::skip]
pub(super) const INITIAL_BIPAI_WITHOUT_HONGBAOPAI_3: [Tile; NUM_BIPAI_3] = [
    tile!(1m), tile!(1m), tile!(1m), tile!(1m),
    tile!(9m), tile!(9m), tile!(9m), tile!(9m),

    tile!(1p), tile!(1p), tile!(1p), tile!(1p),
    tile!(2p), tile!(2p), tile!(2p), tile!(2p),
    tile!(3p), tile!(3p), tile!(3p), tile!(3p),
    tile!(4p), tile!(4p), tile!(4p), tile!(4p),
    tile!(5p), tile!(5p), tile!(5p), tile!(5p),
    tile!(6p), tile!(6p), tile!(6p), tile!(6p),
    tile!(7p), tile!(7p), tile!(7p), tile!(7p),
    tile!(8p), tile!(8p), tile!(8p), tile!(8p),
    tile!(9p), tile!(9p), tile!(9p), tile!(9p),

    tile!(1s), tile!(1s), tile!(1s), tile!(1s),
    tile!(2s), tile!(2s), tile!(2s), tile!(2s),
    tile!(3s), tile!(3s), tile!(3s), tile!(3s),
    tile!(4s), tile!(4s), tile!(4s), tile!(4s),
    tile!(5s), tile!(5s), tile!(5s), tile!(5s),
    tile!(6s), tile!(6s), tile!(6s), tile!(6s),
    tile!(7s), tile!(7s), tile!(7s), tile!(7s),
    tile!(8s), tile!(8s), tile!(8s), tile!(8s),
    tile!(9s), tile!(9s), tile!(9s), tile!(9s),

    tile!(1z), tile!(1z), tile!(1z), tile!(1z), // dong
    tile!(2z), tile!(2z), tile!(2z), tile!(2z), // nan
    tile!(3z), tile!(3z), tile!(3z), tile!(3z), // xi
    tile!(4z), tile!(4z), tile!(4z), tile!(4z), // bei
    tile!(5z), tile!(5z), tile!(5z), tile!(5z), // baiban
    tile!(6z), tile!(6z), tile!(6z), tile!(6z), // lufa
    tile!(7z), tile!(7z), tile!(7z), tile!(7z), // hongzhong
];

pub(super) const BIPAI_RED_5P_INDEX_3: usize = (2 + 4) * NUM_SAME_TILE as usize;
pub(super) const BIPAI_RED_5S_INDEX_3: usize = (2 + 9 + 4) * NUM_SAME_TILE as usize;
