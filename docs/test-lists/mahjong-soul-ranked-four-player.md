# Test list: 雀魂段位戦・四人 walking skeleton

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-09
- Updated: 2026-08-09
- Status: Planned（実装開始の明示指示まで選択しない）
- Requirements: `CORE-001`, `CORE-006`, `CORE-007`, `AI-001`, `AI-002`, `AI-003`, `DATA-002`, `DATA-003`, `NFR-001`, `RULE-010`
- ADR / design: [ADR-0001](../adr/0001-event-driven-typed-continuations.md), [ADR-0005](../adr/0005-mahjong-soul-first.md), [domain model](../design/domain-model.md)
- Rule sources / clauses: [雀魂段位戦詳細ルールと補助資料](../references/rule-sources.md#雀魂の補助検証資料)

## Scope

最初の縦切りとして、雀魂段位戦・四人を基準に、固定した `Bipai` から最初の `Zimo` / `Dapai` 意思決定を発行し、検証済み応答で継続を一度だけ再開し、event と replay可能な状態を得るまでを扱う。

このtest listでは、和了計算、`fulu`、局完結、対局進行、段位point、GPU batching、実際の雀魂presetの`verified`化を扱わない。ルール値を必要とする項目は、公式条項または取得済み牌譜へのsource mappingができるまで着手しない。

## Source readiness

- [ ] 雀魂公式ページの取得日時、対象mode、locale、内容hashを記録する。
- [ ] walking skeletonが依存する設定値を`RuleClaim`へmappingする。
- [ ] 牌譜を使う項目はplayer名を匿名化し、source牌譜ID、対象`Round`、source event範囲、編集内容、最小fixture hashを記録する。
- [ ] 同じsource`majsoul-record`をCI外full-record corpusへ登録し、毎回逐次decodeしてLizhiSimと比較できる。
- [ ] Kanachanの挙動だけを期待値にしていないことをreviewする。

## Examples and tests

### Values and configuration

- [ ] 四人用の全`Seat`を構築でき、範囲外seatを構築できない。
- [ ] `TileKind`と赤牌を含む`TileCopy`を区別する。
- [ ] 雀魂四人基準fixtureの牌構成を検証済み値へ変換できる。
- [ ] copy数不足、重複、除外牌混入を拒否し、部分的な状態を返さない。
- [ ] `Bipai`の順序を固定すると配牌と最初の`zimo`が一意に決まる。
- [ ] Property: 配牌、`bingpai`、`Bipai`の間でtile conservationが保たれる。

### Typed suspension

- [ ] 配牌後、最初のactorだけに`Dapai`要求を発行して中断する。
- [ ] 要求はrequest ID、table ID、actor、観測schema、合法action、continuation tokenを持つ。
- [ ] `Observation<Seat>`は他seatの`bingpai`と未公開`TileCopy`を含まない。
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
- Why: Phase 0中であり、ユーザーから振る舞いの実装開始を指示されていないため。実装開始時は「四人用の全`Seat`を構築でき、範囲外seatを構築できない」を最初の候補とし、さらに小さくできるか確認する。

## Cycle log

実装開始後、選択した一項目についてのみ`red -> green -> refactor`を記録する。

## Completion review

- [ ] すべての項目が完了または理由付きで後続test listへ移送されている。
- [ ] 要求IDとsource clauseに漏れがない。
- [ ] errorと境界を確認した。
- [ ] property/model/contract testの要否を判断した。
- [ ] replayとschema/versionへの影響を確認した。
