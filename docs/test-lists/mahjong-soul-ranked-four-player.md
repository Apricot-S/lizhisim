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

#### `Bingpai` mutation boundary

- [x] 外部crateは任意の空`Bingpai`を公開constructorから構築できない。
- [x] 外部crateは`with_added`または同等の低水準追加操作を直接呼び出せない。
- [x] 外部crateは`with_removed`または同等の低水準除去操作を直接呼び出せない。
- [x] 配牌による`Bingpai`生成は、検証済み`Bipai::qipai`からだけ行える。
- [ ] 通常ツモに続く打牌、および副露による`Bingpai`更新は、対応する検証済み`Round`遷移からだけ行える。
- [x] 低水準の追加・除去操作はcrate-privateとし、`Round`遷移のunit testから種類別上限と未所持牌除去を引き続き検証できる。
- [ ] Property: 公開された配牌、通常ツモ、打牌、副露の任意の合法遷移列から、phaseと`fulu`数に矛盾する`Bingpai`枚数を作れない。

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
- [x] 構築直後の四人用`Bipai`の王牌を除いた残り枚数は122枚である。
- [x] 配牌の3回の4枚取りでは、各seatが`i * 16 + seat_index * 4 + j`の牌を受け取る。
- [x] 配牌の最後の1枚取りでは、各seatがindex `48 + seat_index`の牌を受け取る。
- [x] 四人全員への13枚配牌後の`Bipai`の王牌を除いた残り枚数は70枚である。
- [x] 親のinitial deal由来の最初の`Zimo`はindex 52の牌を返す。
- [x] 親のinitial deal由来の最初の`Zimo`後の`Bipai`の王牌を除いた残り枚数は69枚である。
- [x] initial deal完了後の連続する`Zimo`はindex 52以降の順序を保つ。
- [x] live wallが尽きた後の通常`Zimo`は`LiveWallExhausted`で失敗する。

#### Dealing and conservation

- [x] `Bipai::qipai`は四人分の未検証な`TileKind`配列ではなく、`[Bingpai; 4]`を返す。
- [ ] 固定した`Bipai`を`qipai`すると、各seatの`Bingpai::counts()`が固定した期待値と一致する。
- [x] `qipai`が返す各`Bingpai`は、元の`Bipai`を検証した`TileSet`と同じ種類別上限を適用する。
- [ ] 検証済み`Bipai`からの配牌は失敗しない変換であり、`qipai`の戻り値を`Result`にしない。
- [x] `qipai`用の変換経路から、任意の未検証countsを持つ`Bingpai`を公開APIで構築できない。
- [ ] 配牌indexの割当規則は、順序を保持しない`Bingpai`の内部表現ではなく、各seatの固定期待countsで検証する。
- [ ] `Bipai`の順序を固定すると、14枚配牌を正規化した最初の`Zimo`が一意に決まる。
- [ ] Property: 配牌、`bingpai`、`Bipai`の間でtile conservationが保たれる。

### Typed suspension

- [ ] 配牌完了後はツモ前typestateになり、この状態だけが通常`Zimo`へ進める。
- [ ] 通常`Zimo`はツモ前typestateを消費し、13枚以下の`Bingpai`と分離した`zimopai`を持つツモ後typestateを返す。
- [ ] ツモ後typestateから通常`Zimo`を連続して行えない。
- [ ] ツモ前typestateから`Dapai`を行えない。
- [ ] ツモ後typestateでは、`zimopai`または`Bingpai`内の合法な牌を`Dapai`できる。
- [ ] `Bingpai`内の牌を`Dapai`した場合は、その牌を除去して`zimopai`を`Bingpai`へ取り込む操作を一つの遷移として行う。
- [ ] `zimopai`を`Dapai`した場合は、`Bingpai`の種類別countsを変更しない。
- [ ] `Chi`または`Peng`成立後は、通常ツモ後とは別の打牌待ちtypestateへ遷移する。
- [ ] `Chi`または`Peng`後の打牌待ちtypestateから通常`Zimo`を行えない。
- [ ] `Chi`または`Peng`後の打牌待ちtypestateでは、副露で消費した牌を除いた`Bingpai`から合法な`Dapai`を一回行える。
- [ ] `Daminggang`成立後は`Chi`または`Peng`後の打牌待ちではなく、嶺上ツモ待ちtypestateへ遷移する。
- [ ] 各遷移は元のtypestateを消費し、同じツモ牌、打牌、または副露を二重適用できない。
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
- [x] `Angang`、`Jiagang`、`wangpai`、`baopai`更新は[王牌・嶺上ツモ・宝牌表示](wangpai-replacement-draw-and-baopai.md)へ移送した。
- [ ] `hule` / `huangpai_pingju`による`RoundResult`。
- [ ] `ben`、`lizhibang`、連荘、終局を含む`TableMatch`。
- [ ] 雀魂牌譜corner caseごとのregression/golden test。

## Current

- Selected: None
- Phase: Awaiting next selection
- Why: 配牌からの`Bingpai`生成と低水準変更APIのcrate-private化を完了した。次は`Round`の最小typestateを一項目ずつ実装する。

## Cycle log

- 2026-08-11: `Bipai::try_new`は検証後に`TileSet`を保持するため、借用を受けて内部cloneするAPIから値を受け取るAPIへrefactorした。`TileSet`を引き続き必要とするか判断できる呼び出し側へcloneの責任を移し、現在の呼び出し箇所では後続利用がないためcloneせず所有権を移した。
- 2026-08-11: `qipai`のseat別counts構築をrefactorした。`0..13`ごとの分岐、除算、剰余を使うfoldから、`seat_offset`を一度計算してbatch開始index `0, 16, 32`の各4枚を走査し、最後にindex `48 + seat_index`の1枚を加える手続きへ変更した。3回の4枚取りと最後の1枚取りをコード構造へ直接対応させ、公開挙動は変更していない。
- 2026-08-11: 将来の`Player` / `Round`遷移から利用する`with_added`、`with_removed`と`BingpaiError`をproductionコードへ戻した。低水準操作は`pub(crate)`のまま、利用開始まで理由付き`expect(dead_code)`をproduction buildだけに適用する。`BingpaiError`は上位遷移errorの型付きsourceにできるようcrate rootから公開した。テスト専用constructorは空状態を明示する`empty`へ改名した。
- 2026-08-11: `qipai`由来の`Bingpai`が元の`TileSet`上限を保持するtestを追加した。`with_added`が上限を1枚緩めるmutantでredを確認し、元の上限参照へ復元してgreenにした。その後、正規生成経路を`qipai`へ限定するため`Bingpai::new`、`with_added`、`with_removed`をcrate-privateへ変更した。既存unit testは同一crate内で低水準操作の境界を引き続き検証する。
- 2026-08-11: 「`Bipai::qipai`は`[Bingpai; 4]`を返す」を選択し、戻り値型の不一致でredを確認した。`Bipai`へ検証済み`TileSet`を保持し、crate-privateな検証済みcounts constructorで四人分の`Bingpai`を構築した。配牌期待値は従来の`TileKind`配列から、index計算を再利用しないseat別固定countsへ変更してgreenにした。
- 2026-08-11: `Bingpai`の任意変更を公開せず、配牌、通常ツモ、打牌、副露の検証済み`Round`遷移だけから更新する境界を追加した。`new`、`with_added`、`with_removed`相当は外部crateへ公開せず、種類別count操作としてcrate内に閉じる。進行phaseは`Bingpai`自身へ重ねず`Round`のtypestateで表し、ツモ前、ツモ後、チー・ポン後の打牌待ち、嶺上ツモ待ちを区別する。`zimopai`は既存方針どおり13枚以下の`Bingpai`と分離する。
- 2026-08-11: `Bipai::qipai`が未検証の`[[TileKind; 13]; 4]`ではなく`[Bingpai; 4]`を返す契約を追加した。各seatの固定counts、元の`TileSet`上限の継承、検証済み牌山からの失敗しない内部変換、未検証countsを受け取る公開constructorを作らないことを独立項目として追跡する。配牌順は`Bingpai`内に保持せず、index割当から得られるseat別multisetを固定期待countsで検証する。
- 2026-08-11: `lingshang_zimo`でもlive wallだけを短縮できるよう、`remaining_count`をcursorからの都度計算ではなく`Bipai`の状態へrefactorした。constructorで122、`qipai`で52減算、通常`zimo`成功ごとに1減算し、公開挙動は維持した。
- 2026-08-11: player set固有値に依存しない`remaining_count`と配牌完了後の`zimo`を`Bipai<P>`の共通implへ移した。136枚の検証と四人分の配牌を担う`try_new`、`qipai`は`FourPlayer`固有implに維持し、三人用の型や挙動は追加していない。
- 2026-08-11: `Bipai::zimo`を`Option`から`Result`へ変更し、通常取得可能な牌がない理由を`BipaiError::LiveWallExhausted`として保持するようにした。`remaining_count`は末尾14枚の`wangpai`を除外し、構築直後122枚、`qipai`後70枚、最初の`zimo`後69枚を返す契約へ更新した。
- 2026-08-11: Four-player `Bipai`の配牌後項目を順に検証した。`qipai`後の残り70枚、最初の`Zimo`が固定fixtureのindex 52に対応する`P5`、その後の残り69枚、連続4回の`Zimo`が`[P5, P5, P6, P6]`となることを各一assertionで確認した。cursorの直接検証は内部実装を拘束するため削除し、`remaining_count`による外部契約へ置き換えた。
- 2026-08-11: 配牌前の`Bipai`から`zimo`できないよう`QipaiPending`/`QipaiCompleted` typestateを導入し、`qipai`だけが配牌後型を返し、その型だけがcheckedな`zimo`を提供する構造にした。
- 2026-08-11: 「配牌の最後の1枚取りでは、各seatがindex `48 + seat_index`の牌を受け取る」を選択した。先行testが実装と同じindex計算式を期待値に使っていたため、3回の4枚取りと最後の1枚取りを、4人分13枚の固定`TileKind`配列を一assertionで比較する一つのtestへ統合した。既存`qipai`でgreenを確認した。
- 2026-08-11: 「配牌の3回の4枚取りでは、各seatが`i * 16 + seat_index * 4 + j`の牌を受け取る」を選択し、`qipai`未定義によるmethod not foundでredを確認した。`Bipai`を消費して4人分の固定長13枚配列とcursor 52の`Bipai`を一括で返す`qipai`を実装し、先頭12枚の配牌順を一assertionでgreenにした。
- 2026-08-11: 配牌を外部の任意index取得へ委ねず、`Bipai::qipai`で原子的に行う方針をdomain modelとglossaryへ反映した。13枚目のindex対応と配牌後cursorは後続testで独立に検証する。
- 2026-08-11: 「構築直後の四人用`Bipai`の王牌を除いた残り枚数は122枚である」を選択し、`remaining_count`未定義によるmethod not foundでredを確認した。constructorのcursorを配牌前の0とし、通常ツモ可能枚数を返す最小実装でgreenにした。
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
