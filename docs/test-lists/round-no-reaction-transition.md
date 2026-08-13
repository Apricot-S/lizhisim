# Test list: `Round`の反応なし遷移

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-14
- Updated: 2026-08-14
- Status: Active
- Requirements: `CORE-001`, `CORE-006`, `CORE-007`, `CORE-008`
- ADR / design: [ADR-0001](../adr/0001-event-driven-typed-continuations.md)、[domain model](../design/domain-model.md)
- Related lists: [`Round`の最初の`Dapai`](round-first-dapai.md)、[雀魂段位戦・四人 walking skeleton](mahjong-soul-ranked-four-player.md)
- Rule sources / clauses: 通常打牌後の次手番は打牌者の次のseatである。副露・和了候補と競合解決はこのlistでは扱わない。

## Scope

四人用の打牌後`Round`から、反応なしという解決済み結果を適用して、次actorのツモ前typestateへ
進む最小遷移を扱う。`Seat`は`TableMatch`中の固定位置のままとし、次actorは固定seat順で一つ進める。

この段階の`no_reaction`は、各playerがpassを応答したことを表さない。副露・和了候補、call window、
request、応答、優先順位をまだモデル化しないため、反応なしと確定した結果を受ける内部遷移である。
後続の反応解決は、この結果を生成してから同じ次actorのツモ前状態へ接続する。

このlistでは`Chi`、`Peng`、`Rong`、槓、流局、合法action候補、AI request、応答検証、event、replayを
実装しない。`Bipai`の通常`zimo`位置や`Player`の手牌更新規則も既存実装を変更しない。

## Responsibility boundary

- `Round<DapaiCompleted>`だけが、反応なしという解決済み結果を受けて次手番へ進める。
- 遷移は打牌者以外のplayerや`Bipai`を更新せず、actorだけを固定seat順の次playerへ変更する。
- 戻り値は`Round<FourPlayer, ZimoPending>`とし、次の通常`zimo`は既存のツモ前遷移が担う。
- 将来のcall windowは、反応候補と応答を解決してからこの遷移と同じ結果を選ぶ。候補が存在しないことを
  `Round`内で先回りして判定しない。

## Examples and tests

### 反応なしから次手番へ

- [x] 打牌者がseat 2のとき、反応なし遷移後のactorはseat 3であり、戻り値はツモ前typestateである。
- [x] 四つの開始seatすべてで、反応なし遷移は固定seat順に一つ進み、seat 3の次はseat 0になる。
- [x] 反応なし遷移は`Bipai`と全playerの状態を変更しない。
- [x] 反応なし遷移後の通常`zimo`は、次actorに牌山の次の通常ツモ牌を渡す。

### Typestate

- [x] 打牌後typestateだけが反応なし遷移を提供する。
- [x] 反応なし遷移後のツモ前typestateから、反応なし遷移を連続して行えない。

### Later: 反応解決

- [ ] `Chi`、`Peng`、`Daminggang`、`Rong`の候補を、打牌後の一つのcall windowへ集める。
- [ ] 反応候補がない場合だけ、call windowの解決結果として次actorのツモ前状態へ進む。
- [ ] 複数の応答を到着順ではなくルール上の優先順位で解決する。
- [ ] pass、遅延、重複、未知、cancel済み応答を型付きで区別する。

## Current

- Selected: なし
- Phase: Complete
- Why: 反応なし遷移の最小縦切りは完了した。call window、反応候補、応答解決は後続listで扱う。

## Cycle log

- 2026-08-14: `Round`の最初の`Dapai`縦切りの後継として作成した。副露・和了・call windowを先行実装せず、反応なしという解決済み結果から次actorのツモ前typestateへ移る最小遷移を先に固定する。最初のtestはseat 2からseat 3への一例に限定し、次項目で四seatの循環を三角測量する。
- 2026-08-14: `no_reaction`未定義によるcompile errorをredとして確認した。`FirstZimoOrigin`は親の第一ツモだけに使うrule差分用の型であり、通常巡目の取得元へ一般化しない。
- 2026-08-14: `no_reaction_advances_actor_from_seat_two_to_seat_three`を追加した。`Round<FourPlayer, DapaiCompleted>`だけに`no_reaction(self) -> Round<FourPlayer, ZimoPending>`を追加し、seat 2の打牌後にseat 3へ進むことをgreenにした。`FirstZimoOrigin`は局共通のrule contextとして`Round`が保持し、親第一打の特例は同値と親actor・`first_turn_eligible`の組合せだけで判定する。初回・通常巡目を別typestateにせず、共通の`zimo`・打牌遷移を重複させない。
- 2026-08-14: `no_reaction_advances_every_seat_in_fixed_order`を追加した。各seatを親として実際に`zimo`と`dapai`を行い、反応なし後のactor列を`[1, 2, 3, 0]`と一assertionで比較した。seat 2からseat 3の最小例に対する三角測量であり、既存の剰余演算による実装でgreenとなった。
- 2026-08-14: `no_reaction_preserves_bipai_and_players`を追加した。打牌後の`Bipai`と全playerをcloneした組と、`no_reaction`後の組を一assertionで比較し、actor以外の共通状態を値のまま移送する既存実装でgreenとなった。
- 2026-08-14: `no_reaction_advances_actor_from_seat_two_to_seat_three`を削除した。`no_reaction_advances_every_seat_in_fixed_order`が同じ実遷移列でseat 2からseat 3を含む全4通りとseat 3からseat 0の境界を検証するため、単独testを残しても独立した契約を増やさない。
- 2026-08-14: 牌山index 53を`Z4`へ差し替え、親の通常ツモ・打牌・反応なし後の通常`zimo`が次actorへ`Z4`を渡すtestを追加する。既存の通常`zimo`遷移がこの経路で未検証のため、まず回帰testとしてred/greenを確認する。
- 2026-08-14: `no_reaction_then_zimo_gives_next_wall_tile_to_next_actor`を追加した。index 53を`Z4`へ差し替え、seat 2の通常ツモ・`Moqie`・反応なし後にseat 3が`Z4`を`zimopai`として受け取る組を一assertionで比較した。既存の`Bipai::zimo`と`Round<ZimoPending>::zimo`の実装でgreenとなった。
- 2026-08-14: `Round<FourPlayer, DapaiCompleted>`にだけ`no_reaction(self) -> Round<FourPlayer, ZimoPending>`が実装され、他のtypestateには同名methodがないことをAPI reviewで確認した。コンパイラが保証するためcompile-fail testは追加しない。
- 2026-08-14: `no_reaction`の戻り値である`Round<FourPlayer, ZimoPending>`には同名methodがなく、`Round<FourPlayer, DapaiCompleted>`だけに実装されることをAPI reviewで確認した。したがって反応なし遷移を連続して行えず、コンパイラが保証するためcompile-fail testは追加しない。

## Completion review

- [ ] すべての項目が完了または理由付きで移送されている。
- [ ] 固定`Seat`と次手番順の対応を確認した。
- [ ] `Bipai`と全playerが反応なし遷移で変更されないことを確認した。
- [ ] 反応なし後の通常`zimo`が次actorと牌山位置を正しく接続することを確認した。
- [ ] call window、合法action、request、応答、event、replayを後続listへ移送した。
