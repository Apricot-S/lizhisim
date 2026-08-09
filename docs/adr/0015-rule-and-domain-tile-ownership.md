# ADR-0015: 牌構成設定と実行時牌上限の所有crateを分離する

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner
- Relates to: [ADR-0014](0014-facade-and-core-crates.md)

## Context

`Bingpai`は同じ`TileKind`を5枚以上保持してはならないだけでなく、赤牌設定が0枚なら
`M0`等を保持できず、赤牌設定が1枚なら1枚までしか保持できない。通常の5も赤牌枚数に
応じて上限が変わる。この制約は`Bingpai`だけからは決定できず、解決済みの卓内ルールを
必要とする。

ルール設定の大部分はbool、整数、閉じたenumで表現できる。一方、牌構成の実行時検証では
core domainの`TileKind`と同じ37種類のindexを使う必要がある。`lizhisim-core`が
`lizhisim-rules`へ依存すると、domainが設定schema、preset metadata、出典管理へ引かれ、
アーキテクチャの外側から内側への依存方向に反する。

## Decision

牌構成を次の二種類へ分ける。

1. `lizhisim-rules`はauthoringと監査のための設定を所有する。
   - rawな赤牌枚数、除外牌、総牌数
   - schema、semantic、capability validation
   - `TableRules`、`ValidatedRuleSet<P>`、preset metadata、出典、内容hash
2. `lizhisim-core`は実行時不変条件のための`TileSet`を所有する。
   - `TileKind`の内部indexに対応する37種類の最大枚数
   - 総牌数
   - 各countが物理的に表現可能であること等、サービスに依存しない不変条件

`lizhisim-rules`はraw設定を解決し、`lizhisim-core`が提供する検証済みconstructorを通して
`TileSet`を生成する。雀魂四人の赤牌設定が各1枚なら、`M0/P0/S0`は各1、
`M5/P5/S5`は各3、その他は各4として解決する。

依存方向は次とする。

```text
lizhisim -> lizhisim-rules, lizhisim-core
lizhisim-rules -> lizhisim-core
lizhisim-core -X-> lizhisim-rules
```

卓内runtimeは`ValidatedRuleSet<P>`全体をcore遷移へ渡さず、そこから抽出した`TileSet`と、
遷移が必要とする小さなdomain policy値だけを渡す。これによりcoreはpreset identity、
serialization、source metadataを知らない。

`Bingpai`の追加検証は`TileSet`の対象kind上限を使う。`Bingpai`が`TileSet`を所有するか、
追加時に参照を受け取るかはperformance測定前に固定しない。どちらの場合も、公開APIから
ルール未検証の追加経路を提供しない。

`TileSet`は`Bingpai`と`Bipai`の双方が使うため、`bingpai` moduleの内部型にしない。
`lizhisim-core/src/tile_set.rs`が実装と局所unit testを所有し、crate rootから公開する。
`Bingpai`はkindごとの所持上限、`Bipai`は生成・読込時の完全なmultiset一致、配牌と
replay検証はtile conservationの基準として同じ`TileSet`を使う。

## Consequences

### Positive

- raw設定と実行時domain不変条件の責務が分かれる。
- coreがrule schema、preset、出典管理へ依存しない。
- rulesは必要な境界だけで`TileKind`と`TileSet`を利用できる。
- 三人麻雀の除外牌も同じ37種類の`TileSet`で表現できる。
- `Bingpai`、`Bipai`、配牌、tile conservationが同じ解決済み牌構成を共有できる。

### Negative

- `lizhisim-rules -> lizhisim-core`のcrate依存が増える。
- raw設定型と実行時`TileSet`の二つをmappingする必要がある。
- `ValidatedRuleSet<P>`をそのままcoreへ渡す場合より、必要なdomain policy値の抽出処理が増える。

## Alternatives considered

### `TileSet`を`lizhisim-rules`だけに置く

Rejected. `Bingpai`等のcore型が牌上限を検証するためにrules crateへ依存し、domainが外側の
設定schemaとpreset責務へ引かれる。

### 牌構成をすべて`lizhisim-core`に置く

Rejected. raw schema、出典、preset差分、semantic validationはdomain状態ではなくrulesの責務である。

### `Bingpai`へ赤牌枚数だけを渡す

Rejected. 三人麻雀の除外牌や将来の牌構成差分を別経路で検証することになり、
`Bipai`とtile conservationの基準も重複する。

### uncheckedな`Bingpai`追加APIを残す

Rejected. 学習用シミュレーションでは不正状態がtrajectoryへ混入する損失が大きい。
必要性がbenchmarkで確認されるまでは、公開unchecked APIを設けない。

## Follow-up / verification

- `lizhisim-core`に`TileSet`のtest listを作る。
- `lizhisim-rules` crateを追加し、最小の赤牌設定から`TileSet`を解決するtest listを作る。
- 雀魂四人の各色赤1枚から37 countを解決するgolden testを追加する。
- `Bingpai`が`TileSet`上限を超える追加を拒否するtestへ現在の固定4枚testを置き換える。
- `Bipai`構築時にも同じ`TileSet`との完全一致を検証する。
- `TileSet`の所有または参照方式はbenchmark前に決めず、API利用箇所が現れた時点で選択する。
