# 用語集

この文書は、設計文書、コード、schema、protocol で用いる識別子の規範である。日本語の説明文をピンインへ置き換えることが目的ではなく、英字識別子の表記揺れをなくすことが目的である。

## 1. 識別子の原則

1. 非英語の麻雀用語に対応する確立済み中国語用語がある場合、声調記号を除いたピンインを使う。
2. 日本語のローマ字表記と、統一されていない英訳語は使わない。
3. 麻雀固有の対応語がない技術概念は英語を使う。中国語の語を造らない。
4. 新しい麻雀用語が必要な場合、先に「ユーザー決定待ち」表へ行を追加する。識別子欄はユーザーが決めるまで空欄とし、実装を先行しない。
5. Rust の型・variant は PascalCase、field・function・module は snake_case に機械変換する。規範形自体は小文字のピンインで記録する。
6. `Round` は麻雀の「局」専用とする。「節」は `Matchday` とする。
7. 外部formatやcrateのkebab-case IDは指定表記を保ち、RustのtypeはPascalCase、moduleはsnake_caseへ変換する。

`round-robin`、`round trip`、scheduler の `weighted round-robin` は一般的な英語・アルゴリズム名であり、麻雀の `Round` とは別である。

## 2. 確定済みの麻雀用語

| 日本語での説明 | 原語 | 規範形 | Rust 型・variant 例 | 根拠 |
|---|---|---|---|---|
| 局 | — | `Round` | `Round`, `RoundId`, `RoundResult` | ユーザー指定の一般的英語 |
| 手牌の門前部分 | 兵牌 | `bingpai` | `Bingpai` | ユーザー指定 |
| 副露 | 副露 | `fulu` | `Fulu`, `FuluCall` | ユーザー指定 |
| 向聴 | 向聴 | `xiangting` | `XiangtingPort`, `XiangtingResult` | 採用 crate とユーザー指定方針 |
| 和了 | 和了 | `hule` | `HuleContext`, `HuleEvaluation` | 採用予定 crate とユーザー指定方針 |
| 立直 | 立直 | `lizhi` | `LizhiState`, `LizhiDeclared` | プロジェクト名・既存方針 |

### 2.1 牌・行為・進行

以下はユーザー確認済みの規範形である。`tile` は関連 crate や先行実装との整合を優先して英語を採用する明示的な例外である。

| 日本語での説明 | 原語・由来 | 規範形 | Rust 型・variant 例 |
|---|---|---|---|
| 牌種（赤牌を区別する37種類） | 英語（例外） | `tile` | `TileKind` |
| 牌山 | 壁牌 | `bipai` | `Bipai` |
| 摸牌・ツモ動作 | 自摸 | `zimo` | `Zimo` |
| 打牌 | 打牌 | `dapai` | `Dapai` |
| 摸切・ツモ切り | 摸切 | `moqie` | `Moqie`、`moqie: bool` |
| チー | 吃 | `chi` | `Chi` |
| ポン | 碰 | `peng` | `Peng` |
| 暗槓 | 暗槓 | `angang` | `Angang` |
| 大明槓 | 大明槓 | `daminggang` | `Daminggang` |
| 加槓 | 加槓 | `jiagang` | `Jiagang` |
| ロン | 栄 | `rong` | `Rong` |
| 荒牌平局 | 荒牌平局 | `huangpai_pingju` | `HuangpaiPingju` |
| 北抜き | 抜北 | `babei` | `Babei` |
| 王牌 | 王牌 | `wangpai` | `Wangpai` |
| 宝牌・ドラ | 宝牌 | `baopai` | `Baopai` |
| 里宝牌・裏ドラ | 里宝牌 | `li_baopai` | `LiBaopai` |
| 紅宝牌・赤ドラ | 紅宝牌 | `hong_baopai` | `HongBaopai` |
| 抜北宝牌・抜きドラ | 抜北宝牌 | `babei_baopai` | `BabeiBaopai` |
| 場 | 場 | `chang` | `Chang` |
| 本場 | 本 | `ben` | `Ben` |
| 立直棒・供託 | 立直棒 | `lizhibang` | `Lizhibang` |
| 親・荘家 | 荘家 | `zhuangjia` | `Zhuangjia` |
| 子・散家 | 散家 | `sanjia` | `Sanjia` |
| 場風 | 圏風牌 | `quanfengpai` | `Quanfengpai` |
| 自風 | 門風牌 | `menfengpai` | `Menfengpai` |
| 河・捨て牌列 | 河 | `he` | `He` |
| 責任払い | 包 | `bao` | `Bao` |

### 2.2 牌譜

| 日本語での説明 | 規範形 | Rust 型・module例 | 運用 |
|---|---|---|---|
| 雀魂の生牌譜 | `majsoul-record` | `MajsoulRecord`, `majsoul_record` | 牌譜URLのresponse bytesを保存した`.bin`。service固有のprotobuf event列 |
| LizhiSimの牌譜形式 | `game_log` | `GameLog`, `game_log` | `majsoul-record`から作る場合はagent名を匿名化。通常生成時はagent名を匿名化しない |

## 3. 確定済みの英語用語

麻雀固有の用語ではない architecture/competition 概念に使う。

| 日本語での説明 | 識別子 | 意味 |
|---|---|---|
| 実験 | `Experiment` | 設定、モデル、複数大会、出力を束ねる実行単位。 |
| 大会 | `Competition` | 段位戦 season、league、tournament など、複数対局と順位更新を束ねる最上位単位。 |
| ステージ | `Stage` | 予選、regular、semifinal、final など、同じ進行・集計方針を共有する区間。 |
| 節 | `Matchday` | 同一節として扱う `TableAssignment` の集合。暦上の一日と一致しなくてもよい。 |
| 卓割り | `TableAssignment` | 参加者、座順、対局条件を一つの卓へ割り当てた計画。 |
| 対局 | `TableMatch` | 卓割りから実行される東風戦・東南戦等の単位。文脈なしに `Game` と呼ばない。 |
| 座席 | `Seat` | 卓内の位置。麻雀用語の翻字ではなく汎用の位置概念として扱う。 |
| 行為主体 | `Actor` | 意思決定要求へ応答する主体。 |
| 卓内ルール | `TableRules` | 牌、行為、和了、点数、流局など一局中の意味論。 |
| 対局ルール | `MatchRules` | 開始点、場構成、連荘、延長、終了、精算の意味論。 |
| 大会方針 | `CompetitionPolicy` | schedule、卓割り、集計、進出、持越し、penalty の意味論。 |
| 段位方針 | `RankingPolicy` | 参加資格、matching、段位 point、昇降段の意味論。 |
| プリセット | `Preset` | 出典と版を持つ、検証済みまたは検証中の解決済み設定。 |
| 可動 alias | `current alias` | 現行版を指す便宜名。実験記録には保存しない。 |
| 生設定 | `RawRuleSpec` | file または外部入力から読んだ未検証の値。 |
| 検証済み設定 | `ValidatedRuleSet<P>` | player set を含む全不変条件を満たし、実行可能な設定。 |
| ドメインイベント | `DomainEvent` | 既に起きた事実。命令ではなく過去形の意味を持つ。 |
| 要求 | `Request<R>` | 外部効果または意思決定を求め、応答型 `R` を指定する値。 |
| 中断点 | `Suspension<R, K>` | 応答 `R` と、再開先を表す継続 `K` を保持する停止状態。 |
| 継続 | `Continuation<R>` | 応答を一度だけ消費して次の状態遷移を再開する、data として表した制御。 |
| 応答窓 | `CallWindow` | 一つの原因 event に対する複数 seat の応答を集め、まとめて解決する単位。 |
| 観測 | `Observation` | 特定 seat に合法的に見える情報だけの model 入力。 |
| 完全 view | `OmniscientView` | debug・検証専用の全情報。観測と同じ型にしない。 |
| 合法手 | `LegalActionSet` | ある意思決定点で受理可能な action の有限集合。 |
| 推論 broker | `InferenceBroker` | 互換な意思決定要求を集約し、推論 backend へ batch で渡す shell component。 |
| trajectory | `Trajectory` | 学習または評価用に射影した観測、action、報酬、結果の系列。 |
| 再生 | `Replay` | 記録された外部入力と event を用いて状態を再構築・検証する処理。 |
| 安定 hash | `StateHash` | canonical 表現から計算し、同じ意味状態の一致確認に使う値。 |

## 4. 使用禁止・廃止表記

| 表記 | 理由 | 代替 |
|---|---|---|
| `Hand`（局の意味） | 一般的な `Round` と競合し、手牌の意味とも曖昧 | `Round` |
| `Round`（節の意味） | `Round` は局専用 | `Matchday` |
| `ConcealedHand`, `ClosedHand` | 英訳が統一されず、手牌全体との境界も曖昧 | `bingpai` |
| `Furo`, `Fuuro`, `Meld`（副露の意味） | ローマ字・英訳の表記揺れ | `fulu` |
| `Shanten` | 日本語ローマ字 | `xiangting` |
| `WinEvaluation`（和了評価の意味） | 対応する麻雀用語がある | `hule` 系識別子 |
| `Riichi` | 日本語ローマ字 | `lizhi` |

既存文献や外部 API の固有名を引用する場合は原表記を保ってよいが、LizhiSim の識別子として再利用しない。

## 5. ユーザー決定待ち

`TileKind`の実装前に、37種類のRust variant識別子を決める。識別子欄はユーザーが決めるまで空欄とし、実装を先行しない。

| 対象 | 種類 | 規範形 / Rust variant | 備考 |
|---|---:|---|---|
| 萬子1〜9 | 9 | `M1`〜`M9` | compact notationの候補は`M1`〜`M9` |
| 筒子1〜9 | 9 | `P1`〜`P9` | compact notationの候補は`P1`〜`P9` |
| 索子1〜9 | 9 | `S1`〜`S9` | compact notationの候補は`S1`〜`S9` |
| 赤`5m`、赤`5p`、赤`5s` | 3 | `M0`, `P0`, `S0` | 候補は`M5Red`、`P5Red`、`S5Red` |
| 東、南、西、北 | 4 | `Z1`、`Z2`、`Z3`、`Z4` | ピンイン候補は`Dong`、`Nan`、`Xi`、`Bei` |
| 白、發、中 | 3 | `Z5`、`Z6`、`Z7` | ピンイン候補は`Bai`、`Fa`、`Zhong` |

## 6. 曖昧語を避ける

- 「半荘」は通常 `TableMatch` の一種であり、東風戦も同じ型の別設定とする。
- 「ゲーム」は `Competition`、`TableMatch`、`Round` のいずれにも解釈できるため、設計用語として単独使用しない。
- 英語の `round` を日程単位の意味で使わない。一般アルゴリズム名を除き、大文字小文字にかかわらず局を意味する。
