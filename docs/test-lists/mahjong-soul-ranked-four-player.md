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
- [x] 範囲外errorは失敗したindexと四人用seat数を保持する。
- [x] `TileKind`は赤牌を含む37種類を定義する。
- [x] `TileKind`の内部indexは通常34種類の後に`M0`、`P0`、`S0`を置く。
- [x] `M0`、`P0`、`S0`を`hong_baopai`と判定する。
- [x] `M0`、`P0`、`S0`以外を`hong_baopai`と判定しない。

#### `bingpai` count state

`bingpai`の実装は`TileKind`の内部indexに対応する`[u8; 37]`を用いる。
テストではmemory layoutそのものではなく、外部から観測できる枚数状態を検証する。

- [x] 空の`bingpai`では37種類すべての所持枚数が0である。
- [x] 空の`bingpai`へ`M1`を1枚加えると、`M1`の所持枚数が1になる。
- [x] 空の`bingpai`へ`M1`を1枚加えても、ほかの36種類の所持枚数は変化しない。
- [x] 同じ`TileKind`を複数枚加えると、その所持枚数が加えた枚数になる。
- [x] 各`TileKind`の最大枚数以上に追加しようとすると、上限超過errorで失敗する。
- [x] `M0`を加えると、`M0`の所持枚数が1になる。
- [x] `M0`を加えても、`M5`の所持枚数は増えない。
- [x] `M5`を加えると、`M5`の所持枚数が1になる。
- [x] `M5`を加えても、`M0`の所持枚数は増えない。
- [x] 所持している`TileKind`を1枚除くと、その所持枚数が1減る。
- [x] 所持していない`TileKind`を除こうとすると、対象`TileKind`を含むerrorで失敗する。
- [x] 所持していない`TileKind`を除く操作が失敗した後も、`bingpai`は変化しない。
- [x] `M5`だけを所持するとき、`M0`を除こうとしても`M5`で代替しない。
- [x] `M0`だけを所持するとき、`M5`を除こうとしても`M0`で代替しない。
- [x] 追加元が`M5`だけを持つとき、`M0`の追加要求を`M5`で代替しない。
- [x] 追加元が`M0`だけを持つとき、`M5`の追加要求を`M0`で代替しない。
- [x] 赤牌と通常5を代替しない性質は`P0`と`P5`にも成立する。
- [x] 赤牌と通常5を代替しない性質は`S0`と`S5`にも成立する。

#### Mahjong Soul four-player tile composition

- [ ] 雀魂四人基準fixtureの牌構成は合計136枚になる。
- [ ] 雀魂四人基準fixtureでは`M0`、`P0`、`S0`がそれぞれ1枚になる。
- [ ] 雀魂四人基準fixtureでは`M5`、`P5`、`S5`がそれぞれ3枚になる。
- [ ] 雀魂四人基準fixtureでは5の通常牌以外の31種類がそれぞれ4枚になる。
- [ ] 雀魂四人基準fixtureでは各色の赤い5と通常の5の合計が4枚になる。
- [ ] 合計が136枚でない四人用牌構成を拒否する。
- [ ] 同じ通常34種に属する`TileKind`の合計が4枚を超える牌構成を拒否する。
- [ ] 不正な牌構成を拒否したとき、部分的に構築された状態を返さない。

#### Four-player `Bipai`

四人用`Bipai`は固定長の`[TileKind; 136]`と非公開cursorで表し、
先頭要素の削除や再確保をせずに`Zimo`を進める。
三人用`Bipai`の固定長と型構成は、三人麻雀のtest listを作る時点まで保留する。

- [x] 検証済みの136枚の固定配列から四人用`Bipai`を構築できる。
- [x] 構築直後の四人用`Bipai`の残り枚数は136枚である。
- [x] 配牌の3回の4枚取りでは、各seatが`i * 16 + seat_index * 4 + j`の牌を受け取る。
- [x] 配牌の最後の1枚取りでは、各seatがindex `48 + seat_index`の牌を受け取る。
- [ ] 四人全員への13枚配牌後のcursorは52である。
- [ ] 親のinitial deal由来の最初の`Zimo`はindex 52の牌を返す。
- [ ] 親のinitial deal由来の最初の`Zimo`後のcursorは53である。
- [ ] 親のinitial deal由来の最初の`Zimo`後の`Bipai`の未読部分は83枚である。
- [ ] initial deal完了後の連続する`Zimo`はindex 52以降の順序を保つ。

#### Dealing and conservation

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
- Why: `qipai`が3回の4枚取りを正しいindexから4人へ配ることを確認したため。

## Cycle log

- 2026-08-11: 「配牌の最後の1枚取りでは、各seatがindex `48 + seat_index`の牌を受け取る」を選択した。先行testが実装と同じindex計算式を期待値に使っていたため、3回の4枚取りと最後の1枚取りを、4人分13枚の固定`TileKind`配列を一assertionで比較する一つのtestへ統合した。既存`qipai`でgreenを確認した。
- 2026-08-11: 「配牌の3回の4枚取りでは、各seatが`i * 16 + seat_index * 4 + j`の牌を受け取る」を選択し、`qipai`未定義によるmethod not foundでredを確認した。`Bipai`を消費して4人分の固定長13枚配列とcursor 52の`Bipai`を一括で返す`qipai`を実装し、先頭12枚の配牌順を一assertionでgreenにした。
- 2026-08-11: 配牌を外部の任意index取得へ委ねず、`Bipai::qipai`で原子的に行う方針をdomain modelとglossaryへ反映した。13枚目のindex対応と配牌後cursorは後続testで独立に検証する。
- 2026-08-11: 「構築直後の四人用`Bipai`の残り枚数は136枚である」を選択し、`remaining_count`未定義によるmethod not foundでredを確認した。constructorのcursorを配牌前の0とし、固定配列長から未読枚数を返す最小実装でgreenにした。
- 2026-08-11: `TileSet`との完全multiset一致を検証する既存testを、active listの「検証済みの136枚の固定配列から四人用`Bipai`を構築できる」へ対応付けて完了とした。

- 2026-08-09: 「各`TileKind`の最大枚数以上に追加しようとすると、上限超過errorで失敗する」を選択し、`BingpaiError::TileCountExceeded { tile_kind, max_count }`とcheckedな`with_added`を実装した。検証なしの追加APIは削除し、既存testを`unwrap()`で更新した。
- 2026-08-09: workspace全26 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: `P0/P5`と`S0/S5`の非代替性testを追加した。`rstest`は導入せず、各色を独立したtest functionで検証する。
- 2026-08-09: 赤牌と通常5の相互代替を禁止する4項目を実装した。既存の`with_added`と`with_removed`がkind indexを厳密に使うため、各項目をソース内testとして追加し、結果を一assertionで比較した。
- 2026-08-09: reviewで漏れていた「各`TileKind`の最大枚数以上の追加拒否」を`bingpai` count stateへ追加した。上限値とerror設計はこのtestを選択するcycleで確定する。
- 2026-08-09: 「所持していない`TileKind`を除く操作が失敗した後も、`bingpai`は変化しない」を選択した。元のcount配列を一assertionで比較し、失敗理由の検証とは分離する。
- 2026-08-09: 未所持`M1`除去後に元の空`bingpai`を比較するtestを追加した。`with_removed`は状態をconsumeするためcloneした値へ適用し、元値の不変性を検証した。
- 2026-08-09: 選択testとworkspace全19 testがgreen。Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: `SeatIndexOutOfRange`を`BingpaiError`と比較し、error値の利用箇所に必要な`Debug`、`Error`、`PartialEq`だけを残した。`Clone`、`Copy`、`Eq`、`Hash`は使用されていないため削除した。
- 2026-08-09: `BingpaiError`も同じ方針で`Eq`をderiveせず、`Debug`、`Error`、`PartialEq`だけに統一した。
- 2026-08-09: 次項目として未所持`TileKind`の除去失敗を選択した。失敗理由を保持できる`Result<Bingpai, BingpaiError>`へ`with_removed`の戻り値を変更する。
- 2026-08-09: `BingpaiError::TileNotPresent { tile_kind }`を`thiserror`で追加し、未所持`M1`の除去が対象kind付き`Err`になるtestをgreenにした。既存の所持牌除去testも`Result`へ移行してgreenを維持した。
- 2026-08-09: refactor変更なし。workspace全18 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「所持している`TileKind`を1枚除くと、その所持枚数が1減る」を選択した。`M1`を追加してから除去し、対象countを一assertionで比較する。除去APIは`Option<Bingpai>`を返す。
- 2026-08-09: `with_removed`を`checked_sub`で実装し、所持中の`M1`を除去する選択testをgreenにした。未所持時の失敗挙動は後続testへ残した。
- 2026-08-09: refactor変更なし。workspace全17 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: `M0`と対称な「`M5`を加えると`M5`が1になる」「`M5`を加えても`M0`は増えない」の2項目を選択し、両方を同一cycleでgreenまで進める。
- 2026-08-09: 2項目は既存実装でgreenだったため、`M5`を`M0`へ誤加算するmutantで両方のredを確認し、指定kind自身のindexへ加算する実装へ復元した。
- 2026-08-09: 復元後に2項目とworkspace全16 testがgreen。Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「`M0`を加えても`M5`の所持枚数は増えない」を選択した。`M5`のcountだけを一assertionで比較する。
- 2026-08-09: 選択testは既存実装でgreenだったため、`M0`追加時に`M5`も増やすmutantでredを確認し、指定kindだけを加算する実装へ復元した。
- 2026-08-09: 復元後に選択testとworkspace全14 testがgreen。Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「`M0`を加えると、`M0`の所持枚数が1になる」を選択した。`TileKind::M0.index()`のcountを一assertionで比較する。
- 2026-08-09: 選択testは既存実装でgreenだったため、`M0`を`M5`のindexへ加算するmutantでredを確認し、kind自身のindexへ加算する実装へ復元した。
- 2026-08-09: 復元後に選択testとworkspace全13 testがgreen。`cargo fmt`で整形し、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「同じ`TileKind`を複数枚加えると、その所持枚数が加えた枚数になる」を選択した。`M1`を4回追加し、対象countだけを一assertionで比較する。
- 2026-08-09: 選択testは既存実装でgreenだったため、代入でcountを1へ戻すmutantでredを確認し、加算実装へ復元した。
- 2026-08-09: 復元後に選択testとworkspace全12 testがgreen。Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「空の`bingpai`へ`M1`を1枚加えても、ほかの36種類の所持枚数は変化しない」を選択した。対象範囲を`counts()[1..]`へ限定し、一assertionで検証する。
- 2026-08-09: 選択testは既存実装でgreenだったため、`with_added`がindex + 1へ誤更新するmutantでredを確認し、正しいindex更新へ復元した。
- 2026-08-09: 復元後に選択testとworkspace全11 testがgreen。Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「空の`bingpai`へ`M1`を1枚加えると、`M1`の所持枚数が1になる」を選択した。追加は既存値を消費して新しい`Bingpai`を返すAPIとして検証する。
- 2026-08-09: 選択testを実行し、`with_added`未実装によるmethod not foundで失敗するredを確認した。
- 2026-08-09: `TileKind`へ`#[repr(u8)]`と内部indexを追加し、`Bingpai::with_added`で指定kindのcountだけを増やしてgreenにした。
- 2026-08-09: refactor変更なし。workspace全10 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 「空の`bingpai`では37種類すべての所持枚数が0である」を選択した。`Bingpai`の内部表現は`[u8; 37]`とし、testは全countを一つの配列として比較する一assertionで記述する。
- 2026-08-09: 選択testを実行し、`Bingpai`未実装によるunresolved importで失敗するredを確認した。
- 2026-08-09: 非公開の`[u8; 37]`と読取専用のcount参照だけを持つ`Bingpai`を実装した。追加・除去は未実装のままgreen検証へ進む。
- 2026-08-09: `[u8; 37]`は`Default`をderiveできなかったため、zero-filled arrayを返す`Default`を明示実装して選択testをgreenにした。
- 2026-08-09: refactor変更なし。workspace全9 test、Clippy `-D warnings`、format、build、`git diff --check`が成功した。Cargoのglobal cache使用記録はsandbox外のread-only databaseへ保存できないwarningが出たが、buildとtest結果には影響していない。
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
- 2026-08-09: Rustの変換規約に合わせ、`try_from_index -> Option`を`TryFrom<usize, Error = SeatIndexOutOfRange>`へrefactorした。既存testは一観点・一assertionを維持してgreen。
- 2026-08-09: 次項目として「範囲外errorは失敗したindexと四人用seat数を保持する」を選択し、期待する`Result`全体を比較する一assertionのtestを追加した。
- 2026-08-09: `cargo test -p lizhisim-core seat::tests::out_of_range_error_reports_index_and_seat_count`を実行し、`SeatIndexOutOfRange`に期待fieldがないため失敗するredを確認した。
- 2026-08-09: `thiserror`で`SeatIndexOutOfRange { index, seat_count }`を実装し、選択testをgreenにした。`thiserror 2.0.20`のMIT licenseは`deny.toml`のallow listと既存`THIRD-PARTY-NOTICES.md`に整合する。
- 2026-08-09: refactor変更なし。workspace全4 testとClippy `-D warnings`が成功。ローカルに`cargo-deny` subcommandがないため`cargo deny check`は未実行で、既存のdependency audit CIで検査する。
- 2026-08-09: `Seat`範囲のreviewを完了。四人用の個数、有効index変換、範囲外拒否、診断情報をtest済み。index読出しと`ThreePlayer`は最初のconsumerまたは三人対応まで追加しない。
- 2026-08-09: `TileKind`識別子を確定。赤牌は`M0`、`P0`、`S0`、字牌variantはピンイン、compact notationは`Z1`〜`Z7`とする。
- 2026-08-09: 「`TileKind`は赤牌を含む37種類を定義する」を選択し、`TileKind::ALL`の長さを検証する一assertionのtestを追加した。
- 2026-08-09: `cargo test -p lizhisim-core tile::tests::tile_kind_defines_thirty_seven_kinds`を実行し、`TileKind`未定義のため失敗するredを確認した。
- 2026-08-09: 確定した37 variantと`TileKind::ALL`だけを実装しgreenにした。compact notation変換や赤牌判定は実装していない。
- 2026-08-09: refactor変更なし。workspace全5 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 字牌variantを`Z1`〜`Z7`へ統一し、ピンインをinline commentで保持した。数牌variantと`ALL`は種類ごとの横並びにし、`#[rustfmt::skip]`でlayoutを固定した。既存testはgreenを維持。
- 2026-08-09: 内部indexを通常34種類の後に`M0`、`P0`、`S0`とする方針を決定。canonical serializationの数値表現とはまだ結び付けない。
- 2026-08-09: index順序を`TileKind::ALL`全体で比較する一assertionのtestを選択した。
- 2026-08-09: `cargo test -p lizhisim-core tile::tests::tile_kind_orders_base_kinds_before_red_kinds`を実行し、各色の9直後に赤牌がある現在順との不一致で失敗するredを確認した。
- 2026-08-09: enum宣言と`TileKind::ALL`を通常34種類、`M0`、`P0`、`S0`の順へ変更してgreenにした。refactor変更なし。
- 2026-08-09: workspace全6 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 次項目を「`M0`、`P0`、`S0`を`hong_baopai`と判定する」へ具体化し、3種の判定結果を一つの配列として比較する一assertionのtestを追加した。
- 2026-08-09: `cargo test -p lizhisim-core tile::tests::zero_numbered_suit_kinds_are_hong_baopai`を実行し、`is_hong_baopai`未実装のため失敗するredを確認した。
- 2026-08-09: `M0`、`P0`、`S0`だけに一致する`const fn is_hong_baopai`を実装してgreenにした。refactor変更なし。
- 2026-08-09: workspace全7 test、Clippy `-D warnings`、format、`git diff --check`が成功した。
- 2026-08-09: 次項目を「`M0`、`P0`、`S0`以外を`hong_baopai`と判定しない」へ具体化し、通常34種類の結果を一assertionへ集約したtestを追加した。既存matchが満たすためmutantでredを確認する。
- 2026-08-09: test追加時点ではgreenだった。`M5`も`hong_baopai`とするmutantを一時適用し、通常牌の誤分類で失敗するredを確認してから3種類だけのmatchへ復元した。
- 2026-08-09: 復元後に選択testと全8 testがgreen。refactor変更なし。Clippy `-D warnings`、format、`git diff --check`が成功した。

## Completion review

- [ ] すべての項目が完了または理由付きで後続test listへ移送されている。
- [ ] 要求IDとsource clauseに漏れがない。
- [ ] errorと境界を確認した。
- [ ] property/model/contract testの要否を判断した。
- [ ] replayとschema/versionへの影響を確認した。
