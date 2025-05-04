use crate::point::{
    validate_changbang_point, validate_lizhibang_point, validate_ma, validate_point, Ma, Point,
};
use crate::tile::NUM_SAME_TILE;
use anyhow::{bail, ensure, Result};

#[derive(Clone, PartialEq, Eq)]
pub struct Rule {
    /// Number of Players
    num_player: NumPlayer,
    /// 局数: Rounds
    changshu: Changshu,

    // Points
    /// 開始時持ち点: Default Starting Points
    starting_point: Point,
    /// 1位必要点数: Minimum Points to Win
    min_win_point: Point,
    /// 返し点 (精算原点): Goal
    fandian: Point,
    /// 天辺: Upper Limit
    tianbian: Option<Point>,
    /// 立直供託点数: Riichi Bet Points
    lizhibang_point: Point,
    /// 積み棒点数: Repeat Counter Points
    changbang_point: Point,
    /// ノーテン罰符-1人聴牌: Noten Penalty: 1 Tenpai Player
    noting_fafu_1: Point,
    /// ノーテン罰符-2人聴牌: Noten Penalty: 2 Tenpai Player
    noting_fafu_2: Point,
    /// ノーテン罰符-3人聴牌: Noten Penalty: 3 Tenpai Player
    noting_fafu_3: Point,
    /// 順位・沈みウマ: Uma (Order Bonus)
    shunwei_shen_ma: ShunweiShenMa,
    /// オカ: Oka (Top Prize)
    has_qiu: bool,
    /// 同点同順位: Tied Rank
    allows_tied_rank: bool,
    /// 箱下立直: Riichi for Negative Points
    can_lizhi_for_negative_point: bool,
    /// Deposits when Game Ends with Draw
    lizhibang_at_liuju: LizhibangAtLiuju,

    // Commonly Used Rules
    /// 喰い断: Open Tanyao
    has_shiduan: bool,
    /// 赤ドラ: Red Five
    hongbaopai_count: HongbaopaiCount,
    /// 飛び: Busting
    can_jifei: bool,
    /// 飜縛り: Han Limit
    fan_limit: FanLimit,
    /// 切上げ満貫: Kiriage Mangan
    has_qieshangmanguan: bool,
    /// 頭ハネ: Head-Bump
    has_toutiao: bool,
    /// Tsumo Scoring in 3-Player
    zimo_scoring_3p: ZimoScoring3P,

    // Dora
    /// 表ドラ: Open Dora
    has_biao_baopai: bool,
    /// 裏ドラ: Ura Dora
    has_li_baopai: bool,
    /// 槓ドラ: Kan Dora
    has_gang_biao_baopai: bool,
    /// 槓ドラ即乗り: Reveal Dora immediately after calling Open Kan
    ming_baopai_immediately_open: bool,
    /// 槓裏ドラ: Kan-Ura Dora
    has_gang_li_baopai: bool,
    /// 抜きドラ: Nuki Dora
    has_babei_baopai: bool,

    // Dealer Repeats
    /// 和了連荘: Dealer Repeats on Win
    has_helelianzhuang: bool,
    /// 聴牌/ノーテン連荘: Dealer Repeats if Tenpai/Noten
    tingpai_noting_lianzhuang: TingpaiNotingLianzhuang,
    /// 和了止め: First Place All Last Dealer Win Ends Game
    has_helezhongju: bool,
    /// 聴牌止め: First Place All Last Dealer Tenpai Ends Game
    has_tingpaizhongju: bool,
    /// Multiple Rons involving Dealer
    multiple_rong_with_zhuangjia: MultipleRongWithZhuangjia,

    // In-Match Draws
    /// 四槓散了流局: Four-Kan Abortive Draw
    sigangsanle: Tuzhongliuju,
    /// 四風連打流局: Four-Wind Discarded Draw
    sifenglianda: Tuzhongliuju,
    /// 四家立直流局: Four-Player Riichi Draw
    sijializhi: Tuzhongliuju,
    /// 九種九牌流局: Nine Different Terminals and Honors
    jiuzhongjiupai: Tuzhongliuju,
    /// 三家和了流局: Triple-Ron Draw
    sanjiahele: Tuzhongliuju,

    // Yakuman
    /// 数え役満: Counted Yakuman
    leijiyiman: Leijiyiman,
    /// ダブル役満: Double Yakuman
    has_shuangbeiyiman: bool,
    /// 複合役満: Multiple Yakuman
    has_fuheyiman: bool,
    /// 国士無双の暗槓搶槓: Robbing a Kan for Thirteen Orphans
    has_angang_shisanyao: bool,

    // Others
    /// 人和: Hand of Man
    renhe: Renhe,
    /// 責任払い (役満): Pay Responsibility for Yakuman
    bao_yiman: BaoYiman,
    /// Multiple Pay Responsibility for Yakuman
    can_coexist_multiple_yiman_bao: MultipulYimanBao,
    /// 責任払い (大明槓): Pay Responsibility for After a Kan by Open Kan
    has_bao_daminggang_lingshangkaihua: bool,
    /// 流し満貫: Mangan at Draw
    has_liujumanguan: bool,
    /// 一発: Ippatsu
    has_yifa: bool,
    /// 南入/西入/東入: Extension to South/West/East
    extension_mode: ExtensionMode,
    /// 喰い替え: Swap Calling
    shiti: Shiti,
    /// ツモ番なし立直: Last Turn Riichi
    nonzimo_lizhi: NonzimoLizhi,
    /// 連風牌対子が4符: Double Wind 4 Fu
    is_lianfengpai_4_fu: bool,
    /// 嶺上開花のツモが0符: Tsumo 0 Fu on After a Kan
    is_lingshangkaihua_zimo_0_fu: bool,
    /// Melded Tiles in Tenpai Check
    includes_fulupai_in_tingpai_check: bool,
    /// Concealed Kan after Riichi
    angang_after_lizhi: AngangAfterLizhi,
    /// Noten declaration during Tingpai
    can_noting_declaration: bool,
    /// North is Everyone's Seat Wind in 3-Player
    is_bei_menfengpai: bool,
    /// Dealer's Start Hand is 14 Tiles
    is_zhuangjia_qipai_14: bool,
    /// If Blessing of Heaven occurs, the tile that gives the highest points
    /// is considered the drawn tile, regardless of which tile was actually drawn.
    ignores_actual_zimopai_on_tianhu: bool,
}
// 実装予定で未実装のルール
// 役満の包で該当役満以外の責任も持つか(持つ:天鳳、持たない:雀魂/一番街/Mリーグ)
// 役満包が発生したとき積み棒は誰が払うか(包責者:雀魂/一番街/Mリーグ、放銃者:龍龍)
// 積み棒は包責任者が払うとき、大三元と四槓子の包責者が異なる場合、どちらの包責任者が積み棒を払うか(雀魂: ?(API に配列があるがどっち優先か不明) 一番街: 四槓子(包責者))

#[derive(Clone, PartialEq, Eq)]
pub enum NumPlayer {
    Four = 4,
    Three = 3,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Changshu {
    Yijuzhan = 0,
    Dongfengzhan = 1,
    Banzhuangzhan = 2,
    Yizhuangzhan = 4,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ShunweiMa {
    pub ma_2: Ma,
    pub ma_3: Ma,
    pub ma_4: Ma,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ShunweiShenMa {
    pub shen_0: ShunweiMa,
    pub shen_1: ShunweiMa,
    pub shen_2: ShunweiMa,
    pub shen_3: ShunweiMa,
    pub shen_4: ShunweiMa,
}

#[derive(Clone, PartialEq, Eq)]
pub enum LizhibangAtLiuju {
    TopTakesAll,
    KeepAsDeposit,
}

#[derive(Clone, PartialEq, Eq)]
pub struct HongbaopaiCount {
    pub wanzi: u8,
    pub bingzi: u8,
    pub suozi: u8,
}

#[derive(Clone, PartialEq, Eq)]
pub enum FanLimit {
    Fan1 = 1,
    Fan2 = 2,
    Fan4 = 4,
    Manguan,
    Menqianqing,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ZimoScoring3P {
    ZimoLoss,
    SplitBeijiaPayment,
    FullPayment,
    EqualPayment,
}

#[derive(Clone, PartialEq, Eq)]
pub enum TingpaiNotingLianzhuang {
    Off,
    Tingpailianzhuang,
    Notinglianzhuang,
}

#[derive(Clone, PartialEq, Eq)]
pub enum MultipleRongWithZhuangjia {
    Lianzhuang,
    Toutiao,
    Lunzhuang,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Tuzhongliuju {
    Off,
    Lianzhuang,
    Lunzhuang,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Leijiyiman {
    Off,
    Single,
    Multiple,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Renhe {
    Off,
    Fan4 = 4,
    Fan5 = 5,
    Fan6 = 6,
    Fan8 = 8,
    Fan11 = 11,
    Manguan,
    Tiaoman,
    Beiman,
    Sanbeiman,
    Yiman,
}

#[derive(Clone, PartialEq, Eq)]
pub enum BaoYiman {
    Off,
    DasanyuanDasixi,
    DasanyuanDasixiSigangzi,
}

#[derive(Clone, PartialEq, Eq)]
pub enum MultipulYimanBao {
    OnlyDasanyuan,
    OnlySigangzi,
    Both,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ExtensionMode {
    Off,
    SuddenDeath,
    SuddenDeathWithLianzhuangPriority,
    Fixed4Rounds,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Shiti {
    Forbidden,
    OnlyJinAllowed,
    Allowed,
}

#[derive(Clone, PartialEq, Eq)]
pub enum NonzimoLizhi {
    Forbidden,
    ForbiddenLastTile,
    Allowed,
}

#[derive(Clone, PartialEq, Eq)]
pub enum AngangAfterLizhi {
    Forbidden,
    AllowedIfNotChangingCompositionAndNotDecreasingYaku,
    AllowedIfNotChangingComposition,
    AllowedIfNotChangingWaits,
}

impl Rule {
    #[inline]
    #[must_use]
    pub fn num_player(&self) -> &NumPlayer {
        &self.num_player
    }
    #[inline]
    #[must_use]
    pub fn changshu(&self) -> &Changshu {
        &self.changshu
    }

    // Points
    #[inline]
    #[must_use]
    pub fn starting_point(&self) -> &Point {
        &self.starting_point
    }
    #[inline]
    #[must_use]
    pub fn min_win_point(&self) -> &Point {
        &self.min_win_point
    }
    #[inline]
    #[must_use]
    pub fn fandian(&self) -> &Point {
        &self.fandian
    }
    #[inline]
    #[must_use]
    pub fn tianbian(&self) -> &Option<Point> {
        &self.tianbian
    }
    #[inline]
    #[must_use]
    pub fn lizhibang_point(&self) -> &Point {
        &self.lizhibang_point
    }
    #[inline]
    #[must_use]
    pub fn changbang_point(&self) -> &Point {
        &self.changbang_point
    }
    #[inline]
    #[must_use]
    pub fn noting_fafu_1(&self) -> &Point {
        &self.noting_fafu_1
    }
    #[inline]
    #[must_use]
    pub fn noting_fafu_2(&self) -> &Point {
        &self.noting_fafu_2
    }
    #[inline]
    #[must_use]
    pub fn noting_fafu_3(&self) -> &Point {
        &self.noting_fafu_3
    }
    #[inline]
    #[must_use]
    pub fn shunwei_shen_ma(&self) -> &ShunweiShenMa {
        &self.shunwei_shen_ma
    }
    #[inline]
    #[must_use]
    pub fn has_qiu(&self) -> &bool {
        &self.has_qiu
    }
    #[inline]
    #[must_use]
    pub fn allows_tied_rank(&self) -> &bool {
        &self.allows_tied_rank
    }
    #[inline]
    #[must_use]
    pub fn can_lizhi_for_negative_point(&self) -> &bool {
        &self.can_lizhi_for_negative_point
    }
    #[inline]
    #[must_use]
    pub fn lizhibang_at_liuju(&self) -> &LizhibangAtLiuju {
        &self.lizhibang_at_liuju
    }

    // Commonly Used Rules
    #[inline]
    #[must_use]
    pub fn has_shiduan(&self) -> &bool {
        &self.has_shiduan
    }
    #[inline]
    #[must_use]
    pub fn hongbaopai_count(&self) -> &HongbaopaiCount {
        &self.hongbaopai_count
    }
    #[inline]
    #[must_use]
    pub fn can_jifei(&self) -> &bool {
        &self.can_jifei
    }
    #[inline]
    #[must_use]
    pub fn fan_limit(&self) -> &FanLimit {
        &self.fan_limit
    }
    #[inline]
    #[must_use]
    pub fn has_qieshangmanguan(&self) -> &bool {
        &self.has_qieshangmanguan
    }
    #[inline]
    #[must_use]
    pub fn has_toutiao(&self) -> &bool {
        &self.has_toutiao
    }
    #[inline]
    #[must_use]
    pub fn zimo_scoring_3p(&self) -> &ZimoScoring3P {
        &self.zimo_scoring_3p
    }

    // Dora
    #[inline]
    #[must_use]
    pub fn has_biao_baopai(&self) -> &bool {
        &self.has_biao_baopai
    }
    #[inline]
    #[must_use]
    pub fn has_li_baopai(&self) -> &bool {
        &self.has_li_baopai
    }
    #[inline]
    #[must_use]
    pub fn has_gang_biao_baopai(&self) -> &bool {
        &self.has_gang_biao_baopai
    }
    #[inline]
    #[must_use]
    pub fn ming_baopai_immediately_open(&self) -> &bool {
        &self.ming_baopai_immediately_open
    }
    #[inline]
    #[must_use]
    pub fn has_gang_li_baopai(&self) -> &bool {
        &self.has_gang_li_baopai
    }
    #[inline]
    #[must_use]
    pub fn has_babei_baopai(&self) -> &bool {
        &self.has_babei_baopai
    }

    // Dealer Repeats
    #[inline]
    #[must_use]
    pub fn has_helelianzhuang(&self) -> &bool {
        &self.has_helelianzhuang
    }
    #[inline]
    #[must_use]
    pub fn tingpai_noting_lianzhuang(&self) -> &TingpaiNotingLianzhuang {
        &self.tingpai_noting_lianzhuang
    }
    #[inline]
    #[must_use]
    pub fn has_helezhongju(&self) -> &bool {
        &self.has_helezhongju
    }
    #[inline]
    #[must_use]
    pub fn has_tingpaizhongju(&self) -> &bool {
        &self.has_tingpaizhongju
    }
    #[inline]
    #[must_use]
    pub fn multiple_rong_with_zhuangjia(&self) -> &MultipleRongWithZhuangjia {
        &self.multiple_rong_with_zhuangjia
    }

    // In-Match Draws
    #[inline]
    #[must_use]
    pub fn sigangsanle(&self) -> &Tuzhongliuju {
        &self.sigangsanle
    }
    #[inline]
    #[must_use]
    pub fn sifenglianda(&self) -> &Tuzhongliuju {
        &self.sifenglianda
    }
    #[inline]
    #[must_use]
    pub fn sijializhi(&self) -> &Tuzhongliuju {
        &self.sijializhi
    }
    #[inline]
    #[must_use]
    pub fn jiuzhongjiupai(&self) -> &Tuzhongliuju {
        &self.jiuzhongjiupai
    }
    #[inline]
    #[must_use]
    pub fn sanjiahele(&self) -> &Tuzhongliuju {
        &self.sanjiahele
    }

    // Yakuman
    #[inline]
    #[must_use]
    pub fn leijiyiman(&self) -> &Leijiyiman {
        &self.leijiyiman
    }
    #[inline]
    #[must_use]
    pub fn has_shuangbeiyiman(&self) -> &bool {
        &self.has_shuangbeiyiman
    }
    #[inline]
    #[must_use]
    pub fn has_fuheyiman(&self) -> &bool {
        &self.has_fuheyiman
    }
    #[inline]
    #[must_use]
    pub fn has_angang_shisanyao(&self) -> &bool {
        &self.has_angang_shisanyao
    }

    // Others
    #[inline]
    #[must_use]
    pub fn renhe(&self) -> &Renhe {
        &self.renhe
    }
    #[inline]
    #[must_use]
    pub fn bao_yiman(&self) -> &BaoYiman {
        &self.bao_yiman
    }
    #[inline]
    #[must_use]
    pub fn can_coexist_multiple_yiman_bao(&self) -> &MultipulYimanBao {
        &self.can_coexist_multiple_yiman_bao
    }
    #[inline]
    #[must_use]
    pub fn has_bao_daminggang_lingshangkaihua(&self) -> &bool {
        &self.has_bao_daminggang_lingshangkaihua
    }
    #[inline]
    #[must_use]
    pub fn has_liujumanguan(&self) -> &bool {
        &self.has_liujumanguan
    }
    #[inline]
    #[must_use]
    pub fn has_yifa(&self) -> &bool {
        &self.has_yifa
    }
    #[inline]
    #[must_use]
    pub fn extension_mode(&self) -> &ExtensionMode {
        &self.extension_mode
    }
    #[inline]
    #[must_use]
    pub fn shiti(&self) -> &Shiti {
        &self.shiti
    }
    #[inline]
    #[must_use]
    pub fn nonzimo_lizhi(&self) -> &NonzimoLizhi {
        &self.nonzimo_lizhi
    }
    #[inline]
    #[must_use]
    pub fn is_lianfengpai_4_fu(&self) -> &bool {
        &self.is_lianfengpai_4_fu
    }
    #[inline]
    #[must_use]
    pub fn is_lingshangkaihua_zimo_0_fu(&self) -> &bool {
        &self.is_lingshangkaihua_zimo_0_fu
    }
    #[inline]
    #[must_use]
    pub fn includes_fulupai_in_tingpai_check(&self) -> &bool {
        &self.includes_fulupai_in_tingpai_check
    }
    #[inline]
    #[must_use]
    pub fn angang_after_lizhi(&self) -> &AngangAfterLizhi {
        &self.angang_after_lizhi
    }
    #[inline]
    #[must_use]
    pub fn can_noting_declaration(&self) -> &bool {
        &self.can_noting_declaration
    }
    #[inline]
    #[must_use]
    pub fn is_bei_menfengpai(&self) -> &bool {
        &self.is_bei_menfengpai
    }
    #[inline]
    #[must_use]
    pub fn is_zhuangjia_qipai_14(&self) -> &bool {
        &self.is_zhuangjia_qipai_14
    }
    #[inline]
    #[must_use]
    pub fn ignores_actual_zimopai_on_tianhu(&self) -> &bool {
        &self.ignores_actual_zimopai_on_tianhu
    }

    #[must_use]
    pub fn new(
        num_player: NumPlayer,
        changshu: Changshu,

        // Points
        starting_point: Point,
        min_win_point: Point,
        fandian: Point,
        tianbian: Option<Point>,
        lizhibang_point: Point,
        changbang_point: Point,
        noting_fafu_1: Point,
        noting_fafu_2: Point,
        noting_fafu_3: Point,
        shunwei_shen_ma: ShunweiShenMa,
        has_qiu: bool,
        allows_tied_rank: bool,
        can_lizhi_for_negative_point: bool,
        lizhibang_at_liuju: LizhibangAtLiuju,

        // Commonly Used Rules
        has_shiduan: bool,
        hongbaopai_count: HongbaopaiCount,
        can_jifei: bool,
        fan_limit: FanLimit,
        has_qieshangmanguan: bool,
        has_toutiao: bool,
        zimo_scoring_3p: ZimoScoring3P,

        // Dora
        has_biao_baopai: bool,
        has_li_baopai: bool,
        has_gang_biao_baopai: bool,
        ming_baopai_immediately_open: bool,
        has_gang_li_baopai: bool,
        has_babei_baopai: bool,

        // Dealer Repeats
        has_helelianzhuang: bool,
        tingpai_noting_lianzhuang: TingpaiNotingLianzhuang,
        has_helezhongju: bool,
        has_tingpaizhongju: bool,
        multiple_rong_with_zhuangjia: MultipleRongWithZhuangjia,

        // In-Match Draws
        sigangsanle: Tuzhongliuju,
        sifenglianda: Tuzhongliuju,
        sijializhi: Tuzhongliuju,
        jiuzhongjiupai: Tuzhongliuju,
        sanjiahele: Tuzhongliuju,

        // Yakuman
        leijiyiman: Leijiyiman,
        has_shuangbeiyiman: bool,
        has_fuheyiman: bool,
        has_angang_shisanyao: bool,

        // Others
        renhe: Renhe,
        bao_yiman: BaoYiman,
        can_coexist_multiple_yiman_bao: MultipulYimanBao,
        has_bao_daminggang_lingshangkaihua: bool,
        has_liujumanguan: bool,
        has_yifa: bool,
        extension_mode: ExtensionMode,
        shiti: Shiti,
        nonzimo_lizhi: NonzimoLizhi,
        is_lianfengpai_4_fu: bool,
        is_lingshangkaihua_zimo_0_fu: bool,
        includes_fulupai_in_tingpai_check: bool,
        angang_after_lizhi: AngangAfterLizhi,
        can_noting_declaration: bool,
        is_bei_menfengpai: bool,
        is_zhuangjia_qipai_14: bool,
        ignores_actual_zimopai_on_tianhu: bool,
    ) -> Result<Self> {
        validate_point(&starting_point)?;
        validate_point(&min_win_point)?;
        validate_point(&fandian)?;
        if let Some(p) = tianbian {
            validate_point(&p)?
        };
        validate_lizhibang_point(&lizhibang_point)?;
        validate_changbang_point(&changbang_point)?;
        validate_point(&noting_fafu_1)?;
        validate_point(&noting_fafu_2)?;
        validate_point(&noting_fafu_3)?;

        if fandian < starting_point {
            bail!("Initial Points must be greater or equal to the Starting Points")
        }

        validate_shunwei_ma(&shunwei_shen_ma.shen_0)?;
        validate_shunwei_ma(&shunwei_shen_ma.shen_1)?;
        validate_shunwei_ma(&shunwei_shen_ma.shen_2)?;
        validate_shunwei_ma(&shunwei_shen_ma.shen_3)?;
        validate_shunwei_ma(&shunwei_shen_ma.shen_4)?;

        if hongbaopai_count.wanzi > NUM_SAME_TILE {
            bail!(
                "The number of Red 5m must be between 0 and {}.",
                NUM_SAME_TILE
            )
        }
        if hongbaopai_count.bingzi > NUM_SAME_TILE {
            bail!(
                "The number of Red 5p must be between 0 and {}.",
                NUM_SAME_TILE
            )
        }
        if hongbaopai_count.suozi > NUM_SAME_TILE {
            bail!(
                "The number of Red 5s must be between 0 and {}.",
                NUM_SAME_TILE
            )
        }

        if angang_after_lizhi == AngangAfterLizhi::AllowedIfNotChangingWaits
            && includes_fulupai_in_tingpai_check
        {
            bail!(
                "If the Tenpai determination includes Melds, \
                Concealed Kan after Riichi that changes Hand composition is not allowed."
            )
        }

        if is_zhuangjia_qipai_14 && !ignores_actual_zimopai_on_tianhu {
            bail!(
                "If the number of dealer's start hand is 14, \
                the first draw tile cannot be distinguished."
            )
        }

        Ok(Self {
            num_player,
            changshu,

            // Points
            starting_point,
            min_win_point,
            fandian,
            tianbian,
            lizhibang_point,
            changbang_point,
            noting_fafu_1,
            noting_fafu_2,
            noting_fafu_3,
            shunwei_shen_ma,
            has_qiu,
            allows_tied_rank,
            can_lizhi_for_negative_point,
            lizhibang_at_liuju,

            // Commonly Used Rules
            has_shiduan,
            hongbaopai_count,
            can_jifei,
            fan_limit,
            has_qieshangmanguan,
            has_toutiao,
            zimo_scoring_3p,

            // Dora
            has_biao_baopai,
            has_li_baopai,
            has_gang_biao_baopai,
            ming_baopai_immediately_open,
            has_gang_li_baopai,
            has_babei_baopai,

            // Dealer Repeats
            has_helelianzhuang,
            tingpai_noting_lianzhuang,
            has_helezhongju,
            has_tingpaizhongju,
            multiple_rong_with_zhuangjia,

            // In-Match Draws
            sigangsanle,
            sifenglianda,
            sijializhi,
            jiuzhongjiupai,
            sanjiahele,

            // Yakuman
            leijiyiman,
            has_shuangbeiyiman,
            has_fuheyiman,
            has_angang_shisanyao,

            // Others
            renhe,
            bao_yiman,
            can_coexist_multiple_yiman_bao,
            has_bao_daminggang_lingshangkaihua,
            has_liujumanguan,
            has_yifa,
            extension_mode,
            shiti,
            nonzimo_lizhi,
            is_lianfengpai_4_fu,
            is_lingshangkaihua_zimo_0_fu,
            includes_fulupai_in_tingpai_check,
            angang_after_lizhi,
            can_noting_declaration,
            is_bei_menfengpai,
            is_zhuangjia_qipai_14,
            ignores_actual_zimopai_on_tianhu,
        })
    }
}

fn validate_shunwei_ma(shunwei_ma: &ShunweiMa) -> Result<()> {
    validate_ma(&shunwei_ma.ma_2)?;
    validate_ma(&shunwei_ma.ma_3)?;
    validate_ma(&shunwei_ma.ma_4)?;

    ensure!(
        shunwei_ma.ma_2 >= shunwei_ma.ma_3,
        "Order Bonus of 3rd place cannot be greater than that of the 2nd place",
    );
    ensure!(
        shunwei_ma.ma_3 >= shunwei_ma.ma_4,
        "Order Bonus of 4th place cannot be greater than that of the 3rd place",
    );

    let ma_1 = -1 * (shunwei_ma.ma_2 + shunwei_ma.ma_3 + shunwei_ma.ma_4);
    ensure!(
        ma_1 >= shunwei_ma.ma_2,
        "Order Bonus of 2nd place cannot be greater than that of the 1st place",
    );

    Ok(())
}
