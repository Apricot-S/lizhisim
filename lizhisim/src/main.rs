mod bipai;
mod fulu;
mod he;
mod macros;
mod point;
mod rule;
mod shoupai;
mod tile;

use bipai::{Bipai, RandomBipai};
use mt19937::MT19937;
use rand::SeedableRng;

fn main() {
    let shunwei_ma = rule::ShunweiMa {
        ma_2: 5,
        ma_3: -5,
        ma_4: -15,
    };

    let mjs_rule = rule::Rule::new(
        rule::NumPlayer::Four,
        rule::Changshu::Banzhuangzhan,
        25_000,
        30_000,
        30_000,
        None,
        1_000,
        100,
        1_000,
        1_500,
        1_000,
        rule::ShunweiShenMa {
            shen_0: shunwei_ma.clone(),
            shen_1: shunwei_ma.clone(),
            shen_2: shunwei_ma.clone(),
            shen_3: shunwei_ma.clone(),
            shen_4: shunwei_ma.clone(),
        },
        false,
        false,
        false,
        rule::LizhibangAtLiuju::TopTakesAll,
        true,
        rule::HongbaopaiCount {
            wanzi: 1,
            bingzi: 1,
            suozi: 1,
        },
        true,
        rule::FanfuLimit::Fan1,
        false,
        false,
        rule::ZimoScoring::ZimoLoss,
        true,
        true,
        true,
        false,
        true,
        true,
        true,
        rule::TingpaiNotingLianzhuang::Tingpailianzhuang,
        true,
        true,
        rule::MultipleRongWithZhuangjia::Lianzhuang,
        rule::Tuzhongliuju::Lianzhuang,
        rule::Tuzhongliuju::Lianzhuang,
        rule::Tuzhongliuju::Lianzhuang,
        rule::Tuzhongliuju::Lianzhuang,
        rule::Tuzhongliuju::Off,
        rule::Leijiyiman::Single,
        true,
        true,
        true,
        rule::Renhe::Off,
        rule::BaoYiman::DasanyuanDasixi,
        rule::MultipulYimanBao::Both,
        false,
        true,
        true,
        rule::ExtensionMode::SuddenDeathWithLianzhuangPriority,
        rule::Shiti::Forbidden,
        rule::NonzimoLizhi::Forbidden,
        true,
        false,
        false,
        rule::AngangAfterLizhi::AllowedIfNotChangingWaits,
        false,
        false,
        true,
        true,
    );

    let seed = mt19937::Seed::default();
    let rng = MT19937::from_seed(seed);

    let mut my_bipai = RandomBipai::new(mjs_rule.unwrap(), rng);

    for _p in 0..4 {
        for _i in 0..13 {
            let _ = my_bipai.zimo();
        }
    }

    println!("Left tiles: {}", my_bipai.left_tile_count().unwrap());

    let baopai_indicators = my_bipai.baopai_indicators().clone().unwrap();
    println!("Dora indicator: {}", baopai_indicators[0]);
    let baopai = RandomBipai::to_baopai(&baopai_indicators[0]);
    println!("Dora: {}", baopai);

    let zimopai = my_bipai.zimo();
    println!("Tsumo: {}", zimopai.unwrap());
    println!("Left tiles: {}", my_bipai.left_tile_count().unwrap());
}
