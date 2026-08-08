# Test list: 雀魂段位戦・四人 walking skeleton

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-09
- Updated: 2026-08-09
- Status: Active
- Requirements: `CORE-001`, `CORE-006`, `CORE-007`, `AI-001`, `AI-002`, `AI-003`, `DATA-002`, `DATA-003`, `NFR-001`, `RULE-010`
- ADR / design: [ADR-0001](../adr/0001-event-driven-typed-continuations.md), [ADR-0005](../adr/0005-mahjong-soul-first.md), [domain model](../design/domain-model.md)
- Rule sources / clauses: [雀魂段位戦詳細ルールと補助資料](../references/rule-sources.md#雀魂の補助検証資料)、[最小RuleClaim mapping](../references/mahjong-soul-walking-skeleton-rule-claims.md)

## Scope

最初の縦切りとして、雀魂段位戦・四人を基準に、固定した `Bipai` から最初の `Zimo` / `Dapai` 意思決定を発行し、検証済み応答で継続を一度だけ再開し、event と replay可能な状態を得るまでを扱う。

このtest listでは、和了計算、`fulu`、局完結、対局進行、段位point、GPU batching、実際の雀魂presetの`verified`化を扱わない。ルール値を必要とする項目は、公式条項または取得済み牌譜へのsource mappingができるまで着手しない。

## Source readiness

- [x] 雀魂公式ページの取得日時、対象mode、localeを`SourceReview`へ記録する。原資料の内容hashはverifiedの必須条件にしない。
- [x] walking skeletonが依存する設定値を`RuleClaim`へmappingし、未確定値を`blocked`にする。
- [ ] 牌譜から発見した項目は、必要な遷移を手書きした最小fixtureにし、source牌譜ID、対象`Round`、source event範囲、構成意図、fixture hashを記録する。
- [ ] 同じsource`majsoul-record`をCI外full-record corpusへ登録し、中間`game_log`を必須にせず毎回逐次decode・projectしてLizhiSimと比較できる。
- [ ] 最初の不一致reportに牌譜ID、`Round`、source event index、期待値、実際値、状態要約が含まれる。
- [ ] Kanachanの挙動だけを期待値にしていないことをreviewする。

## Examples and tests

### Values and configuration

- [x] `FourPlayer`は4つの`Seat`を定義する。
- [x] 四人用index 0〜3を対応する`Seat`へ変換できる。
- [x] 四人用index 4を`Seat`へ変換できない。
- [ ] 赤`5m`、赤`5p`、赤`5s`を含む37種類の`TileKind`を構築でき、それ以外の赤牌を構築できない。
- [ ] 雀魂四人基準fixtureの牌構成を検証済み値へ変換できる。
- [ ] `TileKind`ごとの枚数不足、枚数超過、除外牌混入を拒否し、部分的な状態を返さない。
- [ ] `Bipai`の順序を固定すると、14枚配牌を正規化した最初の`Zimo`が一意に決まる。
- [ ] Property: 配牌、`bingpai`、`Bipai`の間でtile conservationが保たれる。

### Typed suspension

- [ ] `RoundStarted`後、initial deal由来の最初の`Zimo`を発行し、親だけに`Dapai`要求を発行して中断する。
- [ ] initial deal由来の牌を最初に`Dapai`した場合、`moqie = false`として記録する。
- [ ] live wall由来の`zimopai`を直後に`Dapai`した場合、`moqie = true`として記録する。
- [ ] 要求はrequest ID、table ID、actor、観測schema、合法action、continuation tokenを持つ。
- [ ] `Observation<Seat>`は他seatの`bingpai`と未公開`TileKind`の個数を含まない。
- [ ] `OmniscientView`は`Observation<Seat>`と型で交換できない。
- [ ] 合法な`Dapai`応答で古い状態を消費し、対応eventを一度だけ発行する。
- [ ] 異なるactor、未知action、schema不一致、未知requestを拒否し、状態を変更しない。
- [ ] 同じ応答を再送してもcontinuationを二重再開しない。

### Events and replay

- [ ] 初期設定、固定`Bipai`、応答から同じevent列を再生成できる。
- [ ] event列から同じ終端状態と`StateHash`を再構築できる。
- [ ] requestの配送時刻やqueueのbatch境界を変えてもevent列が変わらない。
- [ ] Integrity error: event欠落、順序破損、hash不一致を成功replayとして扱わない。

### Later listsへ移送する項目

- [ ] `Chi`、`Peng`、`Daminggang`、`Rong`を競合解決する`CallWindow`。
- [ ] `Angang`、`Jiagang`、`wangpai`、`baopai`更新。
- [ ] `hule` / `huangpai_pingju`による`RoundResult`。
- [ ] `ben`、`lizhibang`、連荘、終局を含む`TableMatch`。
- [ ] 雀魂牌譜corner caseごとのregression/golden test。

## Current

- Selected: None
- Phase: Awaiting next selection
- Why: 選択していた四人用index 4の拒否testがmutantによるred検証を経てgreenとなり、refactor要否の確認まで完了したため。

## Cycle log

- 2026-08-09: `Seat`の構築範囲testを選択。redは未着手。
- 2026-08-09: 局所的不変条件を検証するunit testとして`lizhisim-core/src/seat.rs`へ移動。`tests/`はfacade互換性や複数module間のintegration testに使う。
- 2026-08-09: 「4席ある」「index 0〜3を変換できる」「範囲外を拒否する」の複数観点と複数assertionが一testに混在していたため、test listを三項目へ分割した。
- 2026-08-09: `FourPlayer`が4つの`Seat`を定義する一観点だけを選び、`seat::tests::four_player_defines_four_seats`を一assertionで記述した。
- 2026-08-09: `cargo test -p lizhisim-core seat::tests::four_player_defines_four_seats`を実行し、未実装の`FourPlayer`と`Seat`を参照できないため失敗するredを再確認した。
- 2026-08-09: `FourPlayer`、型付き`Seat<P>`、四席の`Seat::<FourPlayer>::ALL`だけを`lizhisim-core`へ実装し、選択testのgreenを確認した。index変換と範囲外errorは実装していない。
- 2026-08-09: refactorを確認し、追加変更なし。`cargo test --workspace --verbose`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo fmt -- --check`が成功した。
- 2026-08-09: 次項目として「四人用index 0〜3を対応する`Seat`へ変換できる」を選択。一つの配列結果を比較する一assertionのtestを追加した。
- 2026-08-09: `cargo test -p lizhisim-core seat::tests::four_player_seat_converts_every_valid_index`を実行し、`try_from_index`が未実装のため失敗するredを確認した。
- 2026-08-09: `try_from_index`を`Seat::<FourPlayer>::ALL`から安全に検索する最小実装でgreenにした。無効indexから不正な`Seat`を構築しないため、未検証のunchecked castは採用しなかった。
- 2026-08-09: Rust 1.97ではsliceの`get`を`const fn`内で呼べないため、意味上不要な`const`を`try_from_index`から除去した。ほかにrefactor変更なし。
- 2026-08-09: `cargo test --workspace --verbose`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo fmt -- --check`、`git diff --check`が成功した。
- 2026-08-09: 次項目として「四人用index 4を`Seat`へ変換できない」を選択し、一assertionの境界testを追加した。前cycleの安全な検索が既にこの境界を満たすため、mutantでredを確認する。
- 2026-08-09: test追加時点ではgreenだった。`try_from_index`が常に`Some`を返すmutantを一時適用し、index 4が`Some(Seat)`となってtestが失敗するredを確認した後、`ALL.get(index).copied()`へ復元した。
- 2026-08-09: 復元後に選択testと全3 testがgreen。refactor変更なし。workspace test、Clippy `-D warnings`、format、`git diff --check`が成功した。

## Completion review

- [ ] すべての項目が完了または理由付きで後続test listへ移送されている。
- [ ] 要求IDとsource clauseに漏れがない。
- [ ] errorと境界を確認した。
- [ ] property/model/contract testの要否を判断した。
- [ ] replayとschema/versionへの影響を確認した。
