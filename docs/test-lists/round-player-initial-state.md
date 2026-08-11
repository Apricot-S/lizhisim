# Test list: 最小`Player` aggregateと`Round`初期遷移

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-12
- Updated: 2026-08-12
- Status: Active
- Requirements: `CORE-001`, `CORE-002`, `CORE-006`, `CORE-007`, `CORE-008`, `NFR-001`
- ADR / design: [ADR-0001](../adr/0001-event-driven-typed-continuations.md)、[ADR-0012](../adr/0012-normalize-dealer-first-draw.md)、[domain model](../design/domain-model.md)
- Related lists: [雀魂段位戦・四人 walking skeleton](mahjong-soul-ranked-four-player.md)、[TileSetと牌構成rule](tile-set-and-rule-tile-config.md)、[王牌・嶺上ツモ・宝牌表示](wangpai-replacement-draw-and-baopai.md)
- Rule sources / clauses: 固定した四人用`Bipai`によるdomain state testだけを扱う。service固有rule値は参照しない。

## Scope

現在フェーズの四人用`Bipai`から、seatごとの永続状態を所有する最小の`Player`を構築し、
`Round`の配牌前状態を消費してツモ前状態へ遷移し、親の最初の`Zimo`でツモ後状態へ進むまでを扱う。

`Player`は`Bingpai`、`fulu`、`he`を所有する。初期状態の`fulu`と`he`は空の有効値とし、
未初期化を表す`Option`にはしない。`zimopai`、現在actor、進行phaseは`Player`へ置かず、
`Round`のtypestateが所有する。

`He`は`TileKind`と`moqie: bool`を一組にした`Sipai`をAoSとして保持し、
`heapless::Vec<Sipai, 27>`で上限を表す。`fulu`は北抜きと別に管理し、最大4要素とする。

このlistでは`Dapai`、副露、槓、北抜き、立直、和了、鳴き窓、意思決定の型付き中断、
observation、event、replayを実装しない。それらは最初のツモ後状態がgreenになってから、
関連listの項目を一つずつ選択する。

## Responsibility boundary

- `Player`は一つの`Seat`に属する永続状態を所有する。
- `Player`は現在の`Round` phaseや一時的な`zimopai`を所有しない。
- `Round`は`Bipai`、四人分の`Player`、現在actor、phase固有データを所有する。
- `Round`遷移は古い状態を消費し、次のtypestateを返す。
- 任意countsの`Bingpai`や不正な`Seat`から外部crateが`Player`を構築できる公開APIは作らない。
- `fulu`と`he`の操作はこのlistで先行実装せず、空の初期状態と所有先だけを固定する。

## Examples and tests

### Initial `Player` aggregate

- [x] `qipai`由来の`Bingpai`から作った`Player`は、その種類別countsを保持する。
- [x] 四人分の`Player`はindex順に`Seat::<FourPlayer>::ALL`と対応する。
- [x] 配牌直後の各`Player`の`he`は空である。
- [x] 配牌直後の`Player`は`zimopai`を所有しない。実際の所有先は最初の`Zimo` transitionで検証する。
- [x] 外部crateは任意の`Bingpai`、`fulu`、`he`を渡して`Player`を構築できない。
- [ ] Deferred: 配牌直後の各`Player`の`fulu`は空である。構成要素の型を決めてから選択する。

### `Round` qipai transition

- [x] 配牌前の`Round`は`Bipai<FourPlayer, QipaiPending>`と親`Seat`を所有する。
- [x] 配牌前の`Round`には通常`zimo`操作を提供しない。
- [x] `qipai`は配牌前の`Round`を消費し、ツモ前typestateを返す。
- [x] `qipai`後の`Round`は四人分の`Player`を所有する。
- [ ] **Selected:** `qipai`後の最初のactorは親である。
- [ ] `qipai`後の`Round`が所有する`Bipai`の`remaining_count`は70である。
- [ ] Property: `qipai`後の四人分の`Player`と未取得`Bipai`の牌を合計すると元の`TileSet`と一致する。

### First `Zimo` transition

- [ ] 通常`zimo`はツモ前typestateにだけ提供する。
- [ ] 通常`zimo`はツモ前typestateを消費し、ツモ後typestateを返す。
- [ ] 親の最初の`zimopai`は固定`Bipai`のindex 52の牌である。
- [ ] ツモ後typestateでも親の`Bingpai`は13枚のままである。
- [ ] ツモ後typestateは親の`Bingpai`と分離した`zimopai`を一枚だけ所有する。
- [ ] 最初の通常`zimo`後の`Bipai::remaining_count()`は69である。
- [ ] ツモ後typestateから通常`zimo`を連続して行えない。
- [ ] Property: 最初の通常`zimo`後の四人分の`Player`、`zimopai`、未取得`Bipai`の牌を合計すると元の`TileSet`と一致する。

## Later listsへ残す項目

- [ ] `Dapai`による`Bingpai`、`zimopai`、`he`の原子的な更新。
- [ ] Property: 合法な四人用`Round`遷移では`He`の27要素上限を超えない。
- [ ] `Chi`、`Peng`、`Daminggang`による`fulu`と`Bingpai`の更新。
- [ ] `Angang`、`Jiagang`、嶺上ツモ、追加表ドラ表示の順序と権限。
- [ ] 合法action集合、型付き中断、応答検証、continuationの一回消費。
- [ ] public observation、canonical event、replay、終端状態hash。
- [ ] 三人用`Player`集合と三人用`Round`。

## Current

- Selected: `qipai`後の最初のactorは親である。
- Phase: Not started
- Why this is the smallest useful next test: 四人分の`Player`が新しい状態へ保存されることを確認できたため、最初の通常ツモを行うseatを次に固定する。

## Cycle log

- 2026-08-12: 現在フェーズの四人用`Bipai`単体機能完了後の次listとして作成した。`Player`を単独で完成させず、最初の`Round`縦切りに必要な永続状態だけを先に定義する。`Round`のphase差はtypestate、seat固有の永続状態は`Player`、一時的な`zimopai`はツモ後`Round`状態へ置く。
- 2026-08-12: `player_preserves_qipai_bingpai_counts`を追加し、`Player`未定義によるcompile errorをredとして確認した。`Player`には配牌済み`Bingpai`だけを保持させ、crate-privateな`from_qipai`と読み取り用`bingpai`を追加してgreenにした。seat、`fulu`、`he`は後続testまで先行実装しない。
- 2026-08-12: `four_players_follow_seat_all_order`を追加し、`Player`がseatを受け取らず参照APIも持たないcompile errorをredとして確認した。`Player<P>`に`Seat<P>`を必須フィールドとして追加し、`Seat::<FourPlayer>::ALL`と配牌順をzipして構築することでgreenにした。三人用の振る舞いは先行実装していない。
- 2026-08-12: `fulu`は北抜きと別管理して最大4要素、`He`は`Sipai { tile_kind, moqie }`のAoSを`heapless::Vec<Sipai, 27>`で保持する方針とした。`fulu`の構成要素が未確定のため、初期`he`を先に選択した。27要素上限の到達不能性は後続の`Round` property testへ残す。
- 2026-08-12: `player_has_empty_he_after_qipai`を追加し、`Player::he`が存在しないcompile errorをredとして確認した。`heapless::Vec<Sipai, 27>`を内包する`He`を追加し、`Player::from_qipai`が空の`He`を必ず生成する最小実装でgreenにした。`Sipai`は`TileKind`と`moqie: bool`のAoSとした。
- 2026-08-12: 「配牌直後の`Player`は`zimopai`を所有しない」はprivate layoutの不在を実行時testにすると実装詳細へ結合するため、構造reviewで完了とした。`Player`が`seat`、`bingpai`、`he`だけを所有することを確認し、正の実行時保証は後続の「ツモ後typestateは`Bingpai`と分離した`zimopai`を一枚だけ所有する」で行う。
- 2026-08-12: 外部crateからの不正な`Player`構築は、全fieldがprivateで唯一のconstructor `from_qipai`もcrate-privateであることを公開API reviewで確認した。失敗理由を限定できない`compile_fail` testは追加しない。`fulu`の初期状態は構成要素の型が決まるまでdeferし、配牌前`Round`を次に選択した。
- 2026-08-12: `qipai_pending_round_preserves_bipai_and_zhuangjia`を追加し、`Round`未定義のcompile errorをredとして確認した。`Round<P>`に`Bipai<P, QipaiPending>`と`Seat<P>`だけを持たせ、両方の保持を一assertionで確認してgreenにした。後続phaseの汎用化は先行実装していない。
- 2026-08-12: 配牌前`Round`に通常`zimo`を提供しないことをAPI reviewで確認した。`Round<P>`自身に`zimo`はなく、保持する`Bipai<P, QipaiPending>`にも`zimo`が定義されていない。失敗理由を限定できない`compile_fail` testは追加せず、正の遷移testとして`qipai`を次に選択した。
- 2026-08-12: `qipai_consumes_pending_round_and_returns_zimo_pending_round`を追加し、`ZimoPendingRound`と`Round::qipai`未定義のcompile errorをredとして確認した。`qipai(self)`が配牌済み`Bipai`、四人分の`Player`、親を失わずに別型へ移し、戻り型と親の保持を一assertionで確認してgreenにした。現在フェーズに合わせて四人用だけを実装した。
- 2026-08-12: refactorとして`Round<P, State>`へ統一し、親を共通field、`RoundQipaiPending<P>`と`FourPlayerZimoPending`をphase固有payloadとした。field構成が変わらない`Bipai`のmarker typestateとは異なり、`Round`はpayloadによってphaseごとの必須dataだけを保持する。三人用player集合の抽象化は先行していない。
- 2026-08-12: `qipai_round_preserves_four_players`を追加した。直前の`qipai`実装が配牌結果を失わないために四人分の`Player`を既に保持しており、追加時点からgreenだった。`Player::seat()`の配列と`Seat::<FourPlayer>::ALL`を一assertionで比較し、`Round`境界での保存を確認した。production変更はない。

## Completion review

- [ ] すべての項目が完了または理由付きで移送されている。
- [ ] 要求 ID と関連test listに漏れがない。
- [ ] `Player`と`Round`の所有権境界を確認した。
- [ ] 不正状態を公開APIから構築できないことを確認した。
- [ ] property/model-based test の要否を判断した。
- [ ] event、replay、schema/versionへの影響を後続listへ移送した。
