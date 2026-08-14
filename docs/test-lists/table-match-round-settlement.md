# Test list: `TableMatch`の局精算境界

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-14
- Updated: 2026-08-14
- Status: Planned
- Requirements: `MATCH-001`, `MATCH-002`, `MATCH-003`, `MATCH-004`, `CORE-006`, `CORE-007`
- ADR / design: [ADR-0002](../adr/0002-versioned-rule-layers.md)、[ADR-0003](../adr/0003-separate-competition-domain.md)、[domain model](../design/domain-model.md)、[rules and presets](../design/rules-and-presets.md)
- Related lists: [`Round`の最小荒牌平局](round-exhaustive-draw.md)、[雀魂段位戦・四人 walking skeleton](mahjong-soul-ranked-four-player.md)
- Rule sources / clauses: [公式ルール出典台帳](../references/rule-sources.md)で確認する`MatchRules`の連荘、流局、本場、供託、終了・精算条項。未確認の値を既定値にしない。

## Scope

`Round<P, RoundEnded>`を消費し、局外のledgerを更新して次局または`TableMatch`終端を選ぶ
`TableMatchState`と`RoundSettlement`の境界を扱う。`Round`は局内事実だけを保持し、点数、場・局番、
本場、供託、連荘、対局終了を保持しない。

このlistは精算規則そのものを推測しない。荒牌平局の聴牌・ノーテン罰符・親聴牌連荘、和了支払、
all-last、延長、飛び、順位精算は、対応する検証済みruleと後続項目で導入する。

## Responsibility boundary

- `Round`は局内の終端事実と局終了時の`Bipai`・playerを`RoundEnded`として返す。
- `TableMatchState<P>`はseat別点数、場・局番、親、本場、供託、完了局summary、終了文脈を所有する。
- `RoundSettlement<P>`は`RoundEnded`、`TableMatchState`、検証済み`MatchRules`から導出される一局分の更新結果である。
- 次局の牌山の生成とI/Oは精算の外側に置き、`NextRoundSpec<P>`に従う開始境界が決定済みの牌山を渡す。
- `MatchTerminationPolicy`は精算後の状態だけを評価する。和了・流局の分岐へ終了判定を重複させない。

## Examples and tests

### 状態と入力境界

- [ ] `TableMatchState<P>`はplayer setと同じ長さのseat別ledgerを保持する。
- [ ] `RoundEnded`は一度だけ`RoundSettlement`へ消費でき、同じ局を二重精算できない。
- [ ] 局開始時の点数、親、場・局番、本場、供託は明示入力であり、`Round`から推測しない。

### 精算と進行

- [ ] `RoundSettlement`は点数移動、親・本場・供託の更新、次局判断に必要な事実を一つの値として返す。
- [ ] `TableMatchState`は精算結果を適用してからだけ`NextRoundSpec<P>`を返す。
- [ ] 終了判定は精算後の状態だけを入力にし、`Continue`、`Extend`、`Finish`を一意に選ぶ。
- [ ] 次局の牌山は精算結果に含めず、開始境界が決定済みの牌山を受け取る。

### Rule-dependent follow-ups

- [ ] 荒牌平局の聴牌・ノーテン罰符・親聴牌連荘を検証済みruleから適用する。
- [ ] 和了支払、複数和了、本場、供託を`RoundSettlement`へ反映する。
- [ ] 飛び、all-last、延長、アガリ止め、聴牌止め、同点を`MatchTerminationPolicy`で検証する。
- [ ] オカ、ウマ、残供託、順位と`TableMatchResult<P>`を精算する。

### Event and replay follow-ups

- [ ] 局精算と対局終端のcanonical eventをschema version付きで記録する。
- [ ] 精算前後のstable hashと、`TableMatchResult<P>`をreplayで照合する。

## Current

- Selected: None
- Phase: Design complete; implementation not started
- Why: `Round`の最小縦切り完了後に、局内状態と対局進行の責務境界を固定した。最初の実装項目は、対応する`MatchRules`の検証範囲と同時に選択する。

## Cycle log

- 2026-08-14: `Round`の荒牌平局終端後の後続topicとして作成した。`RoundEnded`を局外ledgerへ一度だけ適用する境界、精算後だけに対局進行・終了判定を置くこと、次局の牌山生成を精算から分離することを設計として固定した。rule値が未確認のため、implementationのSelected項目は置かない。

## Completion review

- [ ] すべての精算ruleが検証済み`MatchRules`へ対応付けられている。
- [ ] `Round`と`TableMatchState`で点数・場・局番・本場・供託を二重所有していない。
- [ ] 二重精算、部分更新、未精算の次局開始を型または検証で防ぐ。
- [ ] `TableMatchResult<P>`、event、replay、stable hashの境界を確認した。
