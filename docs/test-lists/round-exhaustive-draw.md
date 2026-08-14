# Test list: `Round`の最小荒牌平局

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-14
- Updated: 2026-08-14
- Status: Active
- Requirements: `CORE-002`, `CORE-006`, `CORE-007`, `CORE-008`
- ADR / design: [ADR-0001](../adr/0001-event-driven-typed-continuations.md)、[domain model](../design/domain-model.md)、[rules and presets](../design/rules-and-presets.md)
- Related lists: [`Round`の反応なし遷移](round-no-reaction-transition.md)、[王牌・嶺上ツモ・宝牌表示](wangpai-replacement-draw-and-baopai.md)、[雀魂段位戦・四人 walking skeleton](mahjong-soul-ranked-four-player.md)
- Rule sources / clauses: [公式ルール出典台帳](../references/rule-sources.md)の各presetで流局・聴牌・連荘を検証する前に、core共通の「通常ツモ牌が尽きた」終端だけを扱う。

## Scope

四人用の通常ツモ牌が尽きた局で、最後の打牌への反応なし後に`Round`を荒牌平局の終端値へ
遷移させる最小縦切りを扱う。副露・和了候補がない現在のwalking skeletonでは、全打牌が反応なしで
解決するため、配牌から70回の通常ツモと打牌を経て一局の終端へ到達できる。

このlistの荒牌平局は「通常ツモ可能なlive wallが0枚である」という事実だけを表す。
聴牌判定、聴牌公開、ノーテン罰符、流し満貫、親聴牌による連荘、供託、本場、次局生成、半荘終了、
event、replay、stable hashは実装しない。これらはrule設定と`RoundSettlement`、`TableMatch`を導入する
後続listで扱う。

## Responsibility boundary

- `Bipai`は通常ツモ可能枚数と、取得不能時の`LiveWallExhausted`を管理する。
- `Round`は打牌後の反応なしを解決し、次ツモ待機またはlive wall枯渇による荒牌平局の終端を返す。
- 終端遷移は終局時の`Bipai`、`Player`、actorを`RoundEnded` typestateへ移送し、部分更新状態を公開しない。
- live wallが残る場合に荒牌平局を確定できない。通常`zimo`との分岐は、同じツモ前状態の枚数で決める。
- 副露・和了を導入した後も、最後の打牌へのcall window解決が反応なしだった場合だけ、この終端へ接続する。

## Examples and tests

### 荒牌平局への到達

- [x] 通常ツモ牌70枚をすべてツモ・打牌・反応なしで消費した後、荒牌平局の終端値を返す。
- [x] 通常ツモ牌が1枚残るツモ前状態では、荒牌平局を確定できない。
- [x] 荒牌平局の終端値は、reasonとして通常ツモ牌枯渇を保持する。
- [x] 終端後に通常`zimo`、`Dapai`、反応なし遷移を提供しない。

### 原子性とtypestate

- [ ] **Selected:** 荒牌平局の終端遷移は、終局時の`Bipai`とplayerを`RoundEnded`へ完全に移送する。
- [ ] 通常ツモ可能枚数が0でない`ZimoPending`から、荒牌平局の終端遷移を拒否する型付きerrorを返す。

### Later: 流局精算と半荘進行

- [ ] 聴牌判定、形式聴牌、聴牌公開、ノーテン罰符を検証済みruleから適用する。
- [ ] 親聴牌・流局種別・連荘ruleから次局specを決定する。
- [ ] 流し満貫を和了、流局精算、不採用のいずれとして扱うかruleで決定する。
- [ ] 本場、供託、得点、順位、半荘終了を`RoundSettlement`後に一か所で決定する。
- [ ] 荒牌平局のcanonical event、observation、replay、終端state hashをversion付きで記録する。

## Current

- Selected: 荒牌平局の終端遷移は、終局時の`Bipai`とplayerを`RoundEnded`へ完全に移送する。
- Phase: Not started
- Why: 終端 reason と終端後 API を型レビューで確認したため、次は終端前後の局内共通状態が値のまま移送されることを検証する。

## Cycle log

- 2026-08-14: 副露・和了の前に一局の最小終端へ到達する後継listとして作成した。ルール固有の流局精算を先行させず、通常ツモ牌枯渇だけを荒牌平局の完了結果として表す。最初のtestは70回の通常ツモ・打牌・反応なしを経た終端に限定し、残り1枚の境界、reason、typestate、原子性を後続項目へ分割する。
- 2026-08-14: 70回の通常ツモ・`Moqie`・反応なしを反復し、最後の反応なし結果が`RoundOutcome::HuangpaiPingju`となるtestを追加する。通常の反応なしと局終端を区別する戻り値型が未定義のため、まずcompile errorをredとして確認する。
- 2026-08-14: `no_reaction_after_all_live_wall_tiles_returns_huangpai_pingju`を追加した。`NoReactionResult<P: PlayerSet + BipaiSpec>`を導入し、通常の反応なしは`NextZimo(Round<P, ZimoPending>)`、live wallが0枚の最後の打牌後は`RoundEnded(Round<P, RoundEnded>)`を返すようにした。70回の通常ツモ・`Moqie`・反応なしを反復し、最後のoutcomeを一assertionで比較してgreenにした。`RoundEnded`は局内の終局理由と終局時の局状態を保持し、精算は後続の`TableMatchState`へ残す。
- 2026-08-14: `no_reaction_with_one_live_wall_tile_returns_next_zimo`を追加した。69回の通常ツモ・`Moqie`・反応なし後の`NoReactionResult`から次ツモ待機を取得し、live wall残り枚数が1であることを一assertionで比較した。既存の`remaining_count() == 0`判定でgreenとなり、production実装の変更は不要だった。
- 2026-08-14: `RoundOutcome::HuangpaiPingju`が通常ツモ牌枯渇による荒牌平局を直接表すことをAPI reviewで確認した。reasonは終端値のvariantであり、追加のtestは不要とした。
- 2026-08-14: `Round<P, RoundEnded>`には`zimo`、`dapai`、`no_reaction`を実装せず、終端後の操作を型で表現不能にしていることをAPI reviewで確認した。コンパイラが保証するためcompile-fail testは追加しない。

## Completion review

- [ ] すべての項目が完了または理由付きで移送されている。
- [ ] live wall枯渇と荒牌平局終端の責務分担を確認した。
- [ ] 終端に聴牌・点数・連荘・次局のrule値を混入させていない。
- [ ] 終端遷移のerrorと部分更新非公開を確認した。
- [ ] 流局精算、半荘進行、event、replay、stable hashを後続listへ移送した。
