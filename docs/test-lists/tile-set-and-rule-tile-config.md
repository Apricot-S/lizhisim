# Test list: `TileSet`と牌構成rule

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-09
- Updated: 2026-08-09
- Status: Active
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

- [x] 37種類のcountから物理的に可能な`TileSet`を構築できる。
- [x] `TileSet`は指定した`TileKind`の最大枚数を返す。
- [x] `TileSet`の総牌数は37 countの合計から一意に求まる。
- [x] 一つの`TileKind`が4枚を超えるcountを拒否する。
- [x] `M0`と`M5`の合計が4枚を超えるcountを拒否する。
- [x] `P0`と`P5`の合計が4枚を超えるcountを拒否する。
- [x] `S0`と`S5`の合計が4枚を超えるcountを拒否する。
- [x] 不正なcountのerrorは対象`TileKind`と上限を保持する。
- [x] 構築失敗時に部分的な`TileSet`を返さない。

### Minimal `lizhisim-rules` boundary

- [x] `lizhisim-rules` crateは`lizhisim-core`へだけ依存して最小scaffoldを構築できる。
- [x] facade `lizhisim`はrulesとcoreの公開APIをre-exportする。
- [x] `M0`のraw枚数を独立に0〜4で受け付ける。
- [x] `P0`のraw枚数を独立に0〜4で受け付ける。
- [x] `S0`のraw枚数を独立に0〜4で受け付ける。
- [x] `M0`のraw枚数5をschema validationで拒否する。
- [x] `P0`のraw枚数5をschema validationで拒否する。
- [x] `S0`のraw枚数5をschema validationで拒否する。
- [x] 赤牌0枚を通常5が4枚、赤牌が0枚の`TileSet`へ解決する。
- [x] 赤3（M/P/S各1枚、合計3枚）を通常5が各3枚の`TileSet`へ解決する。
- [x] 赤牌4枚を通常5が0枚、赤牌が4枚の`TileSet`へ解決する。
- [x] 雀魂四人基準の各色赤1枚を合計136枚の`TileSet`へ解決する。
- [x] 雀魂四人基準では5以外の31種類を各4枚へ解決する。

### Configuration input adapters

- [ ] TOML schemaは赤牌設定を`HongBaopaiConfig`相当の構造としてdecodeできる。
- [ ] TOMLの赤牌枚数0〜4を`RawRuleSpec`へ変換できる。
- [ ] TOMLの赤牌枚数5以上をschema validation errorとして拒否する。
- [ ] TOML decode errorは入力位置または対象fieldを含む。
- [ ] TOMLから解決した`TileSet`はRust APIから直接解決した結果と一致する。
- [ ] TOML入力は未指定fieldのdefaultを暗黙に補完しない。
- [ ] 環境変数overrideを設定経路へ導入しない。

### `Bingpai` integration

- [x] `Bingpai`は`TileSet`で0枚の`TileKind`を追加できない。
- [x] 赤牌1枚の`TileSet`では対応する赤牌を1枚だけ追加できる。
- [x] 赤牌1枚の`TileSet`では対応する通常5を3枚だけ追加できる。
- [x] 赤牌4枚の`TileSet`では対応する通常5を追加できない。
- [x] 上限超過errorは対象`TileKind`、現在枚数、設定上限を保持する。
- [x] 追加失敗後も元の`Bingpai`は変化しない。
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
- Phase: Awaiting next selection
- Why: 赤牌0枚の`RuleSpec`を通常5 4枚・赤牌0枚の`TileSet`へ解決できたため。

## Cycle log

- 2026-08-10: 「赤牌0枚を通常5が4枚、赤牌が0枚の`TileSet`へ解決する」を選択し、M/P/S各0枚の解決結果を一assertionで検証する。既存の`RuleSpec::resolve_tile_set`がすでに契約を満たしていたため、redなしでgreenを確認した。
- 2026-08-10: M/P/Sの赤牌と通常5のcountをまとめて比較する回帰testを追加した。
- 2026-08-10: `P0`と`S0`のraw枚数5拒否をM0と同じサイクルで選択し、対象牌、実枚数、上限を各一assertionで検証した。既存の共通validationが契約を満たしていたため、redなしでgreenを確認した。
- 2026-08-10: `RuleSpecError::HongBaopaiCountOutOfRange`のP0/S0 payload回帰testを追加した。
- 2026-08-10: 「`M0`のraw枚数5をschema validationで拒否する」を選択し、対象牌、実枚数、上限を一assertionで検証する。既存の`HongBaopaiConfig::validate`がすでに契約を満たしていたため、redなしでgreenを確認した。
- 2026-08-10: `RuleSpecError::HongBaopaiCountOutOfRange`のpayloadを検証する回帰testを追加した。
- 2026-08-09: refactorとして`M0CountOutOfRange`、`P0CountOutOfRange`、`S0CountOutOfRange`を`HongBaopaiCountOutOfRange`へ統合し、対象`TileKind`をpayloadへ保持する共通helperを追加した。`const fn`の制約により明示的な`match`でerrorを伝播した。
- 2026-08-09: refactor後もworkspace全37 test、Clippy、format、`git diff --check`が成功した。
- 2026-08-09: rulesの実装を`rule_spec.rs`へ分割し、`lib.rs`を公開APIのre-exportだけにした。`RawRuleSpec`直下の赤牌枚数を`HongBaopaiConfig`へまとめ、`RawRuleSpec -> RuleSpec`の検証境界へ既存3色testを移行した。
- 2026-08-09: refactor後もworkspace全37 test、Clippy、format、`git diff --check`が成功した。
- 2026-08-09: `RawRuleSpec`を外部入力・serde decode用、`RuleSpec`をsemantic validation済み設定とする境界を採用した。serdeのschema validationとrulesのsemantic validationを分離し、TOML入力は後続adapterで扱う。
- 2026-08-09: `validate_hong_baopai_count`を`HongBaopaiConfig::validate`から利用する構造へrefactorし、`RuleSpec`の変換処理を設定検証と構築に限定した。workspace全37 test、Clippy、format、`git diff --check`が成功した。
- 2026-08-09: rulesのmodule分割と`HongBaopaiConfig`導入は、既存behaviorを変えないrefactorとして別cycleで扱う方針を確認した。
- 2026-08-09: TOML/serdeは入力adapterのtest list項目を選択した時点で導入する。domain型へ直接serde deriveを付けず、TOML schemaから`RawRuleSpec`へ変換する境界を置く。環境変数overrideは再現性のため導入しない。
- 2026-08-09: 「`S0`のraw枚数を独立に0〜4で受け付ける」を選択し、`M0 = 0`、`P0 = 0`を固定して0〜4の全入力を一assertionで検証する。
- 2026-08-09: `RawRuleSpec`に`S0`設定が未実装のため、3引数constructorとS0 testをコンパイルできないredを確認した。
- 2026-08-09: `RawRuleSpec`へ`s0_count`と範囲検証を追加し、牌数解決で`S0/S5`へ反映する最小実装をgreenにした。
- 2026-08-09: refactor変更なし。選択testがgreenになった。
- 2026-08-09: 「`P0`のraw枚数を独立に0〜4で受け付ける」を選択し、`M0 = 0`を固定して0〜4の全入力を一assertionで検証する。
- 2026-08-09: `RawRuleSpec`に`P0`設定が未実装のため、2引数constructorとP0 testをコンパイルできないredを確認した。
- 2026-08-09: `RawRuleSpec`へ`p0_count`と範囲検証を追加し、牌数解決で`P0/P5`へ反映する最小実装をgreenにした。
- 2026-08-09: refactor変更なし。選択testがgreenになった。
- 2026-08-09: 「`M0`のraw枚数を独立に0〜4で受け付ける」を選択し、0〜4の全入力を一assertionで検証する。
- 2026-08-09: `RawRuleSpec`と`M0`枚数検証が未実装のため、rules crateのtestをコンパイルできないredを確認した。
- 2026-08-09: `RawRuleSpec::new`と`M0`枚数フィールドを追加し、0〜4を受理する最小実装をgreenにした。`P0/S0`は後続項目に分割した。
- 2026-08-09: refactor変更なし。選択testがgreenになった。
- 2026-08-09: 「`lizhisim-rules` crateは`lizhisim-core`へだけ依存して最小scaffoldを構築できる」を選択した。crate未作成のため、workspaceへ追加してbuildできる状態が未達であることをredの前提として確認する。
- 2026-08-09: workspaceへ`lizhisim-rules`を追加し、`lizhisim-core`だけを依存に持つ空のlibrary scaffoldを作成してgreenにした。
- 2026-08-09: workspace build、全34 test、Clippy、format、`git diff --check`が成功した。
- 2026-08-09: facadeのcore直re-exportを維持し、`lizhisim-rules`を`rules` module内でre-exportする構造にした。workspace全34 test、Clippy、format、`git diff --check`が成功した。
- 2026-08-09: 「構築失敗時に部分的な`TileSet`を返さない」を選択し、`M1 = 4`を含むcountで`Z7 = 5`を拒否し、戻り値が`Err`であることを一assertionで検証する。既存の`Result` APIがすでにこの契約を満たしていたため、redなしでgreenを確認した。
- 2026-08-09: 部分的な`TileSet`を返さない回帰testを追加した。workspace全体の検証は次のrefactor後に実行する。
- 2026-08-09: 「不正なcountのerrorは対象`TileKind`と上限を保持する」を選択し、`Z7 = 5`のerror payloadを一assertionで検証する。既存実装がすでにこの契約を満たしていたため、redなしでgreenを確認した。
- 2026-08-09: `tile_set_count_error_retains_tile_kind_and_max_count`は`tile_set_rejects_kind_count_above_four`と観点が重複するため削除し、後者で対象`TileKind`、実枚数、上限を継続して検証する方針とした。
- 2026-08-09: 「`S0`と`S5`の合計が4枚を超えるcountを拒否する」を選択し、`S0 = 2`、`S5 = 3`の合計5を一assertionで検証する。
- 2026-08-09: `S0/S5`の合計検証が未実装のため、`Ok(TileSet)`になるredを確認した。
- 2026-08-09: `S0`と`S5`の合計が4を超えた場合に既存のcombined-count errorを返す最小実装をgreenにした。
- 2026-08-09: M/P/Sの重複していた赤牌・通常5の合計検証を`validate_combined_five_count`へまとめた。`const fn`の制約により明示的な`match`でerrorを伝播し、workspace全33 test、Clippy、format、`git diff --check`が成功した。
- 2026-08-09: 「`P0`と`P5`の合計が4枚を超えるcountを拒否する」を選択し、`P0 = 2`、`P5 = 3`の合計5を一assertionで検証する。
- 2026-08-09: `P0/P5`の合計検証が未実装のため、`Ok(TileSet)`になるredを確認した。
- 2026-08-09: `P0`と`P5`の合計が4を超えた場合に既存のcombined-count errorを返す最小実装をgreenにした。
- 2026-08-09: refactor変更なし。選択testがgreenになった。
- 2026-08-09: 「`M0`と`M5`の合計が4枚を超えるcountを拒否する」を選択し、`M0 = 2`、`M5 = 3`の合計5を一assertionで検証する。
- 2026-08-09: `TileSetError::CombinedFiveCountExceeded`未定義によるvariant not foundで失敗するredを確認した。
- 2026-08-09: `M0`と`M5`の合計が4を超えた場合に、赤牌、通常5、実枚数、上限を持つerrorを返す最小実装をgreenにした。`P0/P5`と`S0/S5`は後続testへ残した。
- 2026-08-09: refactor変更なし。workspace全31 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「一つの`TileKind`が4枚を超えるcountを拒否する」を選択し、`M1 = 5`が対象kind、実枚数5、上限4を持つerrorになることを一assertionで検証する。
- 2026-08-09: `TileSetError::TileCountExceeded`未定義によるvariant not foundで失敗するredを確認した。
- 2026-08-09: constructorの既存loopで最初に4を超えたcountを検出し、対象`TileKind`、実枚数、上限を持つerrorを返す最小実装をgreenにした。
- 2026-08-09: refactor変更なし。workspace全30 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「`TileSet`の総牌数は37 countの合計から一意に求まる」を選択し、`M1 = 4`、`M2 = 3`の合計7を一assertionで検証する。
- 2026-08-09: `total_count`未実装によるmethod not foundで失敗するredを確認した。
- 2026-08-09: constructor内で37 countを合計して非公開`u16`へ保持し、`TileSet::total_count`で返す最小実装をgreenにした。count配列と総数を別々に入力できないため、不整合な状態は構築できない。
- 2026-08-09: refactor変更なし。workspace全29 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「`TileSet`は指定した`TileKind`の最大枚数を返す」を選択し、`M1`の設定値3を一assertionで検証する。
- 2026-08-09: `max_count`未実装によるmethod not foundで失敗するredを確認した。
- 2026-08-09: `TileKind`の内部indexからcountを返す`TileSet::max_count`だけを実装してgreenにした。refactor変更なし。
- 2026-08-09: workspace全28 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: ADR-0015をAcceptedとし、`TileSet`を独立した`tile_set.rs`へ置く方針でtest listを作成した。
- 2026-08-09: module公開は単独の振る舞いtestにせずdesign constraintとして確認する。最初の項目に「37種類のcountから物理的に可能な`TileSet`を構築できる」を選択した。
- 2026-08-09: `TileSet`と`TileSetError`が未定義のため失敗するredを確認した。
- 2026-08-09: `lizhisim-core/src/tile_set.rs`へ37 countを保持する`TileSet`、将来のvalidation errorを追加する`TileSetError`、最小の`try_from_counts`を実装してgreenにした。上限検証は後続testへ残した。
- 2026-08-09: refactor変更なし。workspace全27 test、Clippy `-D warnings`、format、`git diff --check`が成功した。

- 2026-08-10: 赤3（M/P/S各1枚、合計3枚）と全赤（各4枚）の解決testを追加し、M/P/S各色の赤牌・通常5のcountを各一assertionで検証した。既存の解決実装が契約を満たしていたためredなしでgreenを確認した。

- 2026-08-10: 雀魂四人基準（赤3、各色赤1枚）の`TileSet::total_count() == 136`を検証する回帰testを追加した。既存の解決実装が契約を満たしていたためredなしでgreenを確認した。

- 2026-08-10: 雀魂四人基準の5以外31種類が各4枚となる解決testを追加した。既存の`RuleSpec::resolve_tile_set`が契約を満たしていたためredなしでgreenを確認した。

- 2026-08-10: `Bingpai::new(TileSet)`を追加し、`Bingpai`が常に確定した`TileSet`を値で保持するようにした。公開`Default` traitは削除し、設定上限1枚の`M1`超過testを追加した。workspace全46 test、Clippy、format、`git diff --check`が成功した。

- 2026-08-10: `M1=0`の`TileSet`から`Bingpai`を構築し、追加が`TileCountExceeded(max_count=0)`になるtestを追加した。既存実装が契約を満たしていたためredなしでgreenを確認した。

- 2026-08-10: `M0=1`の`TileSet`で赤牌を1枚追加できるtestを追加した。既存の設定上限利用が契約を満たしていたためredなしでgreenを確認した。

- 2026-08-10: 赤1枚設定で通常5を3枚追加できるtestと、赤4枚設定で通常5が`max_count=0`として拒否されるtestを追加した。既存の`TileSet`上限利用が契約を満たしていたためredなしでgreenを確認した。

- 2026-08-10: 上限到達済み`Bingpai`への追加失敗後、保存した元状態のcountが変化しないtestを追加した。`with_added`の消費型APIとcloneした元状態により契約を確認し、workspace全51 test、Clippy、format、`git diff --check`が成功した。

## Completion review

- [ ] coreとrulesの依存方向がADR-0015に一致する。
- [ ] `TileSet`を`Bingpai`と`Bipai`が共通利用する。
- [ ] 赤牌0、1、4枚の境界を確認した。
- [ ] errorと失敗時不変性を確認した。
- [ ] 四人用牌山の完全multisetを確認した。
- [ ] 三人麻雀固有項目を後続listへ移送した。
