# Test list: `TileSet`と牌構成rule

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-09
- Updated: 2026-08-09
- Status: Planned
- Requirements: `CORE-001`, `CORE-006`, `RULE-001`, `RULE-002`, `NFR-001`, `NFR-003`
- ADR / design: [ADR-0015](../adr/0015-rule-and-domain-tile-ownership.md), [domain model](../design/domain-model.md), [rules and presets](../design/rules-and-presets.md)

## Scope

`lizhisim-core`所有の検証済み`TileSet`、`lizhisim-rules`によるraw牌設定の解決、
`Bingpai`と四人用`Bipai`による共通牌構成の利用を扱う。

役、行為、支払、対局進行等のrule設定、三人麻雀固有の牌構成、牌山shuffleは扱わない。
三人麻雀でも同じ37種類の`TileSet`を使う方針だけを維持し、除外牌の具体testはPhase 4へ送る。

## Design constraints

- `TileSet`は`lizhisim-core/src/tile_set.rs`が所有する。
- `lizhisim-rules`は`lizhisim-core`へ依存し、逆依存は禁止する。
- `TileSet`はraw設定、preset identity、出典、内容hashを保持しない。
- `TileSet`のcount順は`TileKind`の内部index順とする。
- `Bingpai`と`Bipai`は同じ`TileSet`を検証基準に使う。
- 公開unchecked追加・構築APIを設けない。

## Examples and tests

### Core `TileSet`

- [ ] `TileSet`は`tile_set` moduleから公開される。
- [ ] 37種類のcountから物理的に可能な`TileSet`を構築できる。
- [ ] `TileSet`は指定した`TileKind`の最大枚数を返す。
- [ ] `TileSet`の総牌数は37 countの合計から一意に求まる。
- [ ] 一つの`TileKind`が4枚を超えるcountを拒否する。
- [ ] `M0`と`M5`の合計が4枚を超えるcountを拒否する。
- [ ] `P0`と`P5`の合計が4枚を超えるcountを拒否する。
- [ ] `S0`と`S5`の合計が4枚を超えるcountを拒否する。
- [ ] 不正なcountのerrorは対象`TileKind`と上限を保持する。
- [ ] 構築失敗時に部分的な`TileSet`を返さない。

### Minimal `lizhisim-rules` boundary

- [ ] `lizhisim-rules` crateは`lizhisim-core`へだけ依存して最小scaffoldを構築できる。
- [ ] facade `lizhisim`はrulesとcoreの公開APIをre-exportする。
- [ ] `M0`、`P0`、`S0`のraw枚数を独立に0〜4で受け付ける。
- [ ] `M0`のraw枚数5をschema validationで拒否する。
- [ ] `P0`のraw枚数5をschema validationで拒否する。
- [ ] `S0`のraw枚数5をschema validationで拒否する。
- [ ] 赤牌0枚を通常5が4枚、赤牌が0枚の`TileSet`へ解決する。
- [ ] 赤牌1枚を通常5が3枚、赤牌が1枚の`TileSet`へ解決する。
- [ ] 赤牌4枚を通常5が0枚、赤牌が4枚の`TileSet`へ解決する。
- [ ] 雀魂四人基準の各色赤1枚を合計136枚の`TileSet`へ解決する。
- [ ] 雀魂四人基準では5以外の31種類を各4枚へ解決する。

### `Bingpai` integration

- [ ] `Bingpai`は`TileSet`で0枚の`TileKind`を追加できない。
- [ ] 赤牌1枚の`TileSet`では対応する赤牌を1枚だけ追加できる。
- [ ] 赤牌1枚の`TileSet`では対応する通常5を3枚だけ追加できる。
- [ ] 赤牌4枚の`TileSet`では対応する通常5を追加できない。
- [ ] 上限超過errorは対象`TileKind`、現在枚数、設定上限を保持する。
- [ ] 追加失敗後も元の`Bingpai`は変化しない。
- [ ] 現在の固定4枚上限testを`TileSet`由来の上限testへ置き換える。

### Four-player `Bipai` integration

- [ ] 136枚の入力multisetが`TileSet`と完全一致すると四人用`Bipai`を構築できる。
- [ ] 入力の総牌数が`TileSet`より1枚少ない場合を拒否する。
- [ ] 入力の総牌数が`TileSet`より1枚多い場合を拒否する。
- [ ] 総牌数が同じでも一つの`TileKind`が不足する入力を拒否する。
- [ ] 総牌数が同じでも一つの`TileKind`が過剰な入力を拒否する。
- [ ] 赤牌と通常5を入れ替えた入力を同じmultisetとして扱わない。
- [ ] 構築errorは最初に不一致となった`TileKind`、期待枚数、実枚数を保持する。

### Conservation consumers

- [ ] 配牌前の`Bipai` multisetは`TileSet`と一致する。
- [ ] 配牌後の全`bingpai`、未読`Bipai`、最初の`zimopai`の合計は`TileSet`と一致する。
- [ ] replayの各公開check pointで全領域のcount合計が`TileSet`と一致する。

## Later listsへ移送する項目

- [ ] 三人麻雀の`M2`〜`M8`、`M0`除外を`TileSet`へ解決する。
- [ ] 北抜き後の領域間tile conservation。
- [ ] 槓、王牌、嶺上牌、宝牌表示を含む領域間tile conservation。
- [ ] preset metadata、canonical serialization、内容hash。

## Current

- Selected: None
- Phase: Awaiting first selection
- Why: ADR-0015の受入れ後にtest listを作成した段階であり、実装開始項目はまだ選択していないため。

## Cycle log

- 2026-08-09: ADR-0015をAcceptedとし、`TileSet`を独立した`tile_set.rs`へ置く方針でtest listを作成した。

## Completion review

- [ ] coreとrulesの依存方向がADR-0015に一致する。
- [ ] `TileSet`を`Bingpai`と`Bipai`が共通利用する。
- [ ] 赤牌0、1、4枚の境界を確認した。
- [ ] errorと失敗時不変性を確認した。
- [ ] 四人用牌山の完全multisetを確認した。
- [ ] 三人麻雀固有項目を後続listへ移送した。
