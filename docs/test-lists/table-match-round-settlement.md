# Test list: `TableMatch`の局精算境界

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-14
- Updated: 2026-08-14
- Status: In progress
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

- [x] `TableMatchState<P>`はplayer setと同じ長さのseat別ledgerを保持する。
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

- Selected: `RoundEnded`は一度だけ`RoundSettlement`へ消費でき、同じ局を二重精算できない。
- Phase: Planned
- Why: seat別ledgerの明示入力境界を実装した。次は`RoundEnded`の消費を精算結果の生成へ接続する。

## Cycle log

- 2026-08-14: `Round`の荒牌平局終端後の後続topicとして作成した。`RoundEnded`を局外ledgerへ一度だけ適用する境界、精算後だけに対局進行・終了判定を置くこと、次局の牌山生成を精算から分離することを設計として固定した。rule値が未確認のため、implementationのSelected項目は置かない。
- 2026-08-14: 最初の対象としてseat別ledgerの状態境界を選択した。精算規則や終了条件はこのcycleへ混ぜない。
- 2026-08-14: `TableMatchState<FourPlayer>`、`Points`、`Ben`、`RoundIndex`、`Lizhibang`を追加した。4席分のledgerを`[Points; FourPlayer::PLAYER_COUNT]`で保持し、長さ不一致を型で表現不能にした。`PlayerSet::PLAYER_COUNT`を追加し、`Seat<FourPlayer>`とledgerの長さを同じ定数へ寄せた。`Players`の具体型を`player.rs`側に置いて循環依存を避けた。状態の各値が局開始時の明示入力であり、`Round`から推測されないことをテストした。次のcycleでは`RoundEnded`の一回消費を扱う。

## Completion review

- [ ] すべての精算ruleが検証済み`MatchRules`へ対応付けられている。
- [ ] `Round`と`TableMatchState`で点数・場・局番・本場・供託を二重所有していない。
- [ ] 二重精算、部分更新、未精算の次局開始を型または検証で防ぐ。
- [ ] `TableMatchResult<P>`、event、replay、stable hashの境界を確認した。
