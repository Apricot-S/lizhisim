# Test list: 王牌・嶺上ツモ・宝牌表示

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-11
- Updated: 2026-08-11
- Status: Active
- Requirements: `CORE-002`, `CORE-003`, `CORE-005`, `CORE-006`, `CORE-007`, `NFR-001`, `RULE-001`, `RULE-002`, `RULE-010`
- ADR / design: [domain model](../design/domain-model.md), [rules and presets](../design/rules-and-presets.md), [ADR-0013](../adr/0013-tile-kind-without-copy-identity.md)
- Rule sources / clauses: [rule source ledger](../references/rule-sources.md)。service別の槓ドラ公開時点、北抜き、裏ドラ適用条件は実装選択前にsource mappingする。
- Supporting evidence: ユーザー提示のKanachan和了時コードでは、表ドラ表示牌を`131 - i * 2`、裏ドラ表示牌を`130 - i * 2`から取得する。補助資料であり、単独でrule期待値にはしない。

## Scope

四人用136枚`Bipai`の末尾14枚を`wangpai`として扱い、通常`zimo`から分離した嶺上ツモ、
表ドラ表示牌、裏ドラ表示牌の公開状態と反復参照、順序、上限、rule依存の公開時点、tile conservationを扱う。

このlistでは槓の合法性、槍槓の競合、和了計算、点数計算、三人麻雀の北抜き自体の合法性は
実装しない。それらの遷移が発行した検証済みの取得権限を`wangpai`境界が正しく消費することを扱う。
三人用`Bipai`の実装はPhase 4まで行わないが、北抜きruleによる嶺上牌容量の差は後続testとして追跡する。

## Decisions

- core内では各幢の上下を再現せず、固定配列末尾側から決定的なindex列で取得する。
- 四人用の嶺上牌は`135, 134, 133, 132`の順とする。
- 表ドラ表示牌は`131, 129, 127, 125, 123`の順とし、最初の1枚と最大4回の追加表示を表す。
- 裏ドラ表示牌は`130, 128, 126, 124, 122`の順とし、対応する表ドラ表示牌と同数だけ取得する。
- `qipai`自体は初期の表ドラ表示牌を公開しない。上位の`Round`が解決済みpolicyを参照し、表ドラ有効時だけ`qipai`完了後に初期表示commandを適用する。
- 表ドラ無効ruleでは初期表示commandを適用せず、公開済み表ドラ表示牌を0枚のまま保つ。
- 裏ドラ表示牌は和了成立と裏ドラ適用資格の確定後、点数評価に必要な境界でだけ取得可能とする。進行中の通常viewへ公開しない。
- 公開済みの表裏ドラ表示牌は`wangpai`内の牌へのread-only viewであり、何度参照しても公開枚数や取得位置を変えない。
- 追加の表ドラ表示時点は、上位の`Round`遷移が槓種別とruleから決める。`wangpai`は槓種別やruleを解釈しない。
- 槓と北抜きは`lingshang_zimo`の一回限りの権限を発行する。北抜きは追加の表ドラ表示を発生させない。
- 三人麻雀の嶺上牌容量は、北抜き有効ruleでは8枚、北抜き無効ruleでは4枚とする。
- `lingshang_zimo`を1回行うたびに通常`zimo`可能な`remaining_count`も1減らす。物理的な王牌補充によるlive wall短縮を、配列の移動なしでcountへ反映する。
- 通常`zimo`と`lingshang_zimo`で同じindexを二重消費しない。表裏ドラ表示牌は消費せず、対応indexを参照する。

## Responsibility boundary

- `Bipai` / `Wangpai`は、通常`zimo`位置、`lingshang_zimo`位置、公開済み表ドラ枚数、各領域の上限と非重複を管理する。
- `Bipai` / `Wangpai`は表ドラの有効・無効を判断せず、上位から渡された初期表示commandを一度だけ適用する。
- 公開済みの表裏ドラ表示牌一覧は何度でも参照でき、参照操作は状態を変更しない。
- `Round`は、`Angang`、`Jiagang`、`Daminggang`、`Babei`の成立、rule、現在phaseから、次に許可する公開・`lingshang_zimo`を決める。
- 三人用`Bipai` / `Wangpai`は解決済みruleから嶺上牌容量を構成するが、個々の北抜きが合法かどうかは判定しない。
- `Round`は未処理の追加表示件数を保持する。複数の槓に由来する保留を上書きまたは一件へcollapseしない。
- `Wangpai`へ槓種別やrule設定を渡して順序を判断させない。上位遷移が検証済みの一回分commandを渡す。

## Index layout

| 用途 | 取得順のindex | 最大数 |
|---|---|---:|
| 通常`zimo`可能範囲 | `52..=121`（`qipai`後） | 70 |
| 裏ドラ表示牌 | `130, 128, 126, 124, 122` | 5 |
| 表ドラ表示牌 | `131, 129, 127, 125, 123` | 5 |
| 嶺上牌 | `135, 134, 133, 132` | 4 |

index列は用途ごとの論理順であり、物理的な幢の上下をcanonical stateへ再現しない。

## Examples and tests

### Layout and construction

- [x] 四人用`Bipai`の末尾14枚を通常`zimo`可能枚数へ含めない。
- [x] 通常`zimo`可能範囲、嶺上牌、表ドラ表示牌、裏ドラ表示牌のindex集合は互いに重ならない。
- [x] Property: 136枚の各indexは、配牌、通常`zimo`可能範囲、嶺上牌、表ドラ表示牌、裏ドラ表示牌のいずれか一つにだけ分類される。

### Initial baopai indicator

- [x] `qipai`前は初期表示commandを適用できない。
- [x] `qipai`直後の公開済み表ドラ表示牌は0枚である。
- [x] 初期表示commandを適用すると、公開済み表ドラ表示牌はindex 131の`TileKind`一枚になる。
- [x] 初期表示commandを二回適用すると拒否する。
- [x] 公開済みの初期表ドラ表示牌を複数回参照しても、毎回同じindex 131の`TileKind`を返す。
- [x] 公開済み表ドラ表示牌の参照は公開枚数を変えない。
- [x] 公開済み表ドラ表示牌の参照は通常`zimo`の`remaining_count`を変えない。
- [x] 公開済み表ドラ表示牌の参照は通常`zimo`位置を変えない。
- [x] 公開済み表ドラ表示牌の参照は`lingshang_zimo`位置を変えない。
- [x] `qipai`前に表ドラ表示牌の参照APIを提供しない。

### Replacement draw authorization

- [ ] 槓成立後に一回分の嶺上ツモ権限を発行できる。
- [ ] 北抜き成立後に一回分の嶺上ツモ権限を発行できる。
- [ ] 槓または北抜きに由来する権限なしでは嶺上ツモできない。
- [ ] 一つの権限から嶺上ツモを二回行えない。
- [ ] 槓用と北抜き用の権限を、原因を失った同一の無型tokenとして扱わない。
- [ ] 失効済みまたは別の`Round`の権限を拒否する。
- [ ] 拒否後も嶺上牌の次の取得位置は変化しない。

### Replacement draw order and limits

- [x] 最初の嶺上ツモはindex 135の`TileKind`を返す。
- [x] 連続する4回の嶺上ツモはindex `135, 134, 133, 132`の順を保つ。
- [x] 4枚取得後は追加の嶺上ツモを拒否する。
- [x] `lingshang_zimo`を1回行うと通常`zimo`の`remaining_count`も1減る。
- [x] 4回の`lingshang_zimo`後は通常`zimo`の`remaining_count`が4減る。
- [x] `lingshang_zimo`を1回行った後、通常`zimo`はindex 120までで終了し、補充対象となったindex 121を返さない。
- [x] 4回の`lingshang_zimo`後、通常`zimo`はindex 117までで終了し、index 118〜121を返さない。
- [x] 通常`zimo`は嶺上牌の取得位置を変えない。

### Three-player replacement draw capacity

- [ ] 三人麻雀で北抜き有効ruleから構成した`Bipai`は、嶺上牌を8枚持つ。
- [ ] 三人麻雀で北抜き無効ruleから構成した`Bipai`は、嶺上牌を4枚持つ。
- [ ] 北抜き有効ruleでは、検証済みの権限による8回目の`lingshang_zimo`まで成功する。
- [ ] 北抜き有効ruleでは、8枚取得後の9回目の`lingshang_zimo`を拒否する。
- [ ] 北抜き無効ruleでは、検証済みの権限による4回目の`lingshang_zimo`まで成功する。
- [ ] 北抜き無効ruleでは、4枚取得後の5回目の`lingshang_zimo`を拒否する。
- [ ] Property: 三人麻雀の嶺上牌容量は北抜きruleだけで決まり、同じ解決済みruleから常に同じ容量が構成される。

### Additional baopai indicators

- [x] 初期表示後、最初の追加表示はindex 129の`TileKind`を返す。
- [x] 表ドラ表示牌はindex `131, 129, 127, 125, 123`の順を保つ。
- [x] 初期表示を含む5枚の表示後は追加表示を拒否する。
- [ ] **Selected:** 追加表示権限なしでは表ドラ表示牌を増やせない。
- [ ] 一つの槓成立から追加の表ドラ表示牌を二枚公開できない。
- [ ] 北抜き後は表ドラ表示枚数が増えず、追加表示権限も発行しない。
- [ ] 公開済みの複数の表ドラ表示牌は何度参照しても同じ順序と内容を返す。
- [ ] 表ドラ表示牌一覧の参照は未処理の追加表示件数を消費しない。

### Round orchestration and rule-dependent reveal timing

- [ ] 表ドラ無効ruleでは、`Round`は`qipai`後に初期表示commandを適用しない。
- [ ] 表ドラ有効ruleでは、`Round`は`qipai`後に初期表示commandを一回適用する。
- [ ] `Round`はservice名ではなく、rules crateから渡されたcore所有の解決済み表ドラpolicyだけを参照する。
- [ ] `Round`は槓種別とruleから、追加表示と`lingshang_zimo`のどちらを先に行うか決定する。
- [ ] `Angang`用のrule順序を`Jiagang`または`Daminggang`へ暗黙に流用しない。
- [ ] 「嶺上ツモ前」になる槓種別・ruleでは、追加表示を完了するまで`lingshang_zimo`へ進めない。
- [ ] 「嶺上ツモ後」になる槓種別・ruleでは、`lingshang_zimo`を完了するまで追加表示へ進めない。
- [ ] 同じrule、`Bipai`、action列なら、queueやbatch境界を変えても公開event順が変わらない。
- [ ] ruleにない第三の順序や、両方を同時に許す曖昧な状態を構築できない。
- [ ] `Wangpai`の公開・取得APIは槓種別やruleを引数に取らず、上位から渡された一回分commandだけを検証する。

### Consecutive gang reveal backlog

- [ ] `Daminggang`または`Jiagang`に由来する追加表示が保留中でも、後続の`Angang`に由来する追加表示を別件として蓄積する。
- [ ] 天鳳ruleで`Daminggang`または`Jiagang`の後に`Angang`が連続した場合、保留された表ドラ表示牌を2枚連続で公開する。
- [ ] 上記2回の公開はindex 129、127の順で、二つの別eventとして記録する。
- [ ] 保留件数2を一回の公開で0にせず、公開ごとに一件だけ減らす。
- [ ] 連続公開の途中で中断・再開しても、残りの保留件数と次の表示indexを保つ。

### Li baopai indicators

- [ ] 進行中または和了不成立時は裏ドラ表示牌を通常viewから取得できない。
- [ ] 和了成立後でも裏ドラ適用資格がなければ裏ドラ表示牌を取得できない。
- [ ] 裏ドラ適用資格のある和了では、公開済み表ドラ表示牌と同数の裏ドラ表示牌を取得できる。
- [ ] 初期表示だけの場合、裏ドラ表示牌はindex 130の`TileKind`一枚である。
- [ ] 最大4回の追加表示後、裏ドラ表示牌はindex `130, 128, 126, 124, 122`の順を保つ。
- [ ] 裏ドラ表示牌の取得は表ドラ表示枚数、嶺上牌位置、通常`zimo`可能枚数を変えない。
- [ ] 同じ和了評価で裏ドラ表示牌を複数回参照しても、同じ順序と内容を返す。

### Conservation, visibility, and replay

- [ ] 配牌、全`bingpai`、通常`zimopai`、嶺上ツモで取得済みの牌、未取得`Bipai`の所有枚数合計は元の`TileSet`と一致する。
- [ ] 表裏ドラ表示牌は`wangpai`内の牌への参照であり、tile conservationで別の所有牌として二重加算しない。
- [ ] `Observation<Seat>`は未公開の表ドラ表示牌、裏ドラ表示牌、未取得の嶺上牌を含まない。
- [ ] `OmniscientView`だけが検証用に未公開の`wangpai`を参照できる。
- [ ] 表ドラ表示、裏ドラ表示、嶺上ツモは原因actionと取得indexをcanonical eventへ記録する。
- [ ] Replay: 同じrule、`Bipai`、action列から同じ表示牌、嶺上牌、event順を再構築できる。
- [ ] Integrity error: 表示回数、権限消費、取得index、event順の不一致を成功replayとして扱わない。

## Source readiness

- [ ] 雀魂四人の初期表ドラ表示時点を公式資料または牌譜証跡へmappingする。
- [ ] 雀魂四人の暗槓・加槓・大明槓ごとの追加表示時点を牌譜証跡へmappingする。
- [ ] 雀魂三人の北抜き後の`lingshang_zimo`を牌譜証跡へmappingする。
- [ ] 天鳳の`Daminggang`または`Jiagang`から`Angang`が連続する場合の2回連続表示を牌譜または公式挙動へmappingする。
- [ ] 天鳳、麻雀一番街、競技ruleの差分は各preset実装前に個別mappingする。
- [ ] Kanachanのindex挙動だけを期待値にしていないことをreviewする。

## Later listsへ移送する項目

- [ ] 槍槓を含む槓成立前後の競合解決。
- [ ] 四槓散了と四槓子継続の判定。
- [ ] 嶺上開花、海底摸月等の和了contextと役判定。
- [ ] 王牌補充と物理的な幢の上下を再現するservice固有projector。
- [ ] 三人麻雀の北抜き自体の合法性、手牌更新、抜きドラ加算。

## Current

- Selected: 追加表示権限なしでは表ドラ表示牌を増やせない。
- Phase: Red
- Why this is the smallest useful next test: 6枚目を拒否する上限を固定したため、追加表示の許可を上位遷移の一回分commandに限定する境界を検討する。

## Cycle log

- 2026-08-11: 「初期表示を含む5枚の表示後は追加表示を拒否する」を選択した。追加表示操作を`Result`へ変更し、5枚上限で`BaopaiIndicatorLimitReached`を返すようにした。上限比較を`>`へ緩めるmutantで6枚目が`Ok`となるredを確認し、`>=`へ復元してgreenにした。
- 2026-08-11: 「表ドラ表示牌はindex 131, 129, 127, 125, 123の順を保つ」を選択した。初期表示と追加表示4回後の一覧を、各indexへ配置した異なる`TileKind`の固定列として比較した。これにより最初の追加表示index 129のtestを包含するため、単独testはこの5枚列testへ置き換えた。indicator間隔を2から1へ変えるmutantで`M0, Z7, P0, Z6, S0`となるredを確認し、2へ復元してgreenにした。
- 2026-08-11: 「初期表示後、最初の追加表示はindex 129」を選択した。`Bipai<QipaiCompleted>`へcrate-privateな`reveal_additional_baopai_indicator`を追加し、上位の`Round`が発行する一回分commandとして公開枚数を1だけ進める最小実装にした。index 129へ`M0`を固定し、初期表示を除く一覧を一assertionで比較した。追加枚数を2にするmutantでindex 127の牌まで公開されるredを確認し、1へ復元してgreenにした。上限、権限、初期表示前の追加表示は後続項目として分離する。
- 2026-08-11: 136枚全体の分類propertyを追加した。配牌`0..=51`、通常ツモ、嶺上牌、表ドラ、裏ドラの各indexをソートして`0..=135`と比較し、各indexをちょうど一つの用途に分類することを一assertionで検証した。このtestは先行した王牌内4領域の非重複testを完全に包含するため置き換えた。配牌枚数を48へ変えるmutantで48〜51の重複が生じるredを確認し、52へ復元してgreenにした。
- 2026-08-11: 「通常`zimo`可能範囲、嶺上牌、表ドラ表示牌、裏ドラ表示牌のindex集合は互いに重ならない」を選択した。4集合をソートしてindex `52..=135`の連続列と比較し、非重複だけでなく王牌領域全体を漏れなく覆うことを一assertionで検証した。嶺上牌開始indexを131へ変えるmutantで128〜131の重複と132〜135の欠番が生じるredを確認し、135へ復元してgreenにした。
- 2026-08-11: 「`qipai`前の参照失敗後も`Bipai`の状態は変化しない」は、配牌前の`Bipai`に表示牌参照APIが存在しないtypestateと整合しないruntime failure表現だったため、「`qipai`前に表ドラ表示牌の参照APIを提供しない」へ正規化した。`baopai_indicators`は`Bipai<FourPlayer, QipaiCompleted>`だけのimplにあり、`QipaiPending`にはmethodが存在しないことをreviewして完了とした。外部compile-fail testを必要とする公開APIではないため、新dependencyは追加しない。
- 2026-08-11: 「`qipai`前は初期表示commandを適用できない」を選択した。`reveal_initial_baopai_indicator`は`Bipai<P, QipaiCompleted>`だけのimplにあり、`QipaiPending`にはmethodが存在しないことをreviewして完了とした。commandは`pub(crate)`でもあるため、外部integration testでは可視性errorとtypestate errorを分離できず、compile-fail test基盤のない現時点で新dependencyを増やす価値はない。将来commandを外部公開する場合は、外部crateからのcompile-fail testを追加する。
- 2026-08-11: 「公開済み表ドラ表示牌の参照は`lingshang_zimo`位置を変えない」を選択した。公開済み表示牌をiteratorが尽きるまで参照した後の嶺上ツモ4回が`S0, P0, M0, Z7`となることを検証した。表示牌参照は内部可変性を持たない`Bipai`への`&self`だけを受け取り、嶺上ツモの取得回数も通常fieldであるため、前2項目と同じ理由で安全なruntime mutantを作れずredは省略した。共有借用による型上の不変性と後続取得の回帰値を確認してgreenとした。
- 2026-08-11: 「公開済み表ドラ表示牌の参照は通常`zimo`位置を変えない」を選択した。公開済み表示牌をiteratorが尽きるまで参照した後の最初の通常ツモがindex 52の`P5`となることを検証した。表示牌参照は内部可変性を持たない`Bipai`への`&self`だけを受け取り、通常ツモcursorも通常fieldであるため、前項目と同じ理由で安全なruntime mutantを作れずredは省略した。共有借用による型上の不変性と後続取得の回帰値を確認してgreenとした。
- 2026-08-11: 「公開済み表ドラ表示牌の参照は通常`zimo`の`remaining_count`を変えない」を選択した。公開済み表示牌をiteratorが尽きるまで参照した後の`remaining_count`を参照前と比較した。`baopai_indicators`は通常の内部可変性を持たない`Bipai`への`&self`だけを受け取り、`remaining_count`も通常fieldであるため、この変更単位で安全なruntime mutantを作れずredは省略した。共有借用による型上の不変性と公開操作後の回帰値を確認してgreenとした。
- 2026-08-11: 「通常`zimo`は嶺上牌の取得位置を変えない」を選択した。通常ツモを1回挟んだ後も嶺上ツモ4回の列が`S0, P0, M0, Z7`となることを一assertionで検証した。通常ツモcursorを嶺上牌indexへ誤って反映するmutantで`P0, M0, Z7, Z7`となるredを確認し、嶺上取得回数だけを使う実装へ復元してgreenにした。三人用嶺上牌容量はPhase 4対象のため、次は現在の四人用実装だけで検証できる表ドラ表示牌参照の非消費性を選択した。
- 2026-08-11: 「4回の`lingshang_zimo`後、通常`zimo`はindex 117までで終了する」を選択した。66回目の通常ツモをindex 117の`Z4`、その次を`LiveWallExhausted`として一assertionへ集約した。最初の嶺上ツモだけを減算するmutantで次の通常ツモがindex 118の`Z4`となるredを確認し、各成功取得で1減算する実装へ復元してgreenにした。
- 2026-08-11: 「`lingshang_zimo`を1回行った後、通常`zimo`はindex 120までで終了する」を選択した。69回目の通常ツモをindex 120の`Z4`、その次を`LiveWallExhausted`として一assertionへ集約した。嶺上ツモの減算値を0にするmutantで次の通常ツモがindex 121の`Z5`となるredを確認し、1へ復元してgreenにした。
- 2026-08-11: 「`lingshang_zimo`を1回行うと`remaining_count`も1減る」を選択した。減算値を0にするmutantで実際値70となるredを確認し、1へ復元してgreenにした。続く「4回後は4減る」は実装変更なしで進められたため、4回目以降に減算が欠けるmutantで実際値69となるredを確認し、各成功取得で1減算する実装へ復元してgreenにした。前者は最初の取得、後者は反復時の更新を別々に固定する。
- 2026-08-11: 「4枚取得後は追加の`lingshang_zimo`を拒否する」を選択した。上限guardを`>`へ緩めるmutantで5回目が`Ok(..., Z7)`となるredを確認し、`>=`へ復元してgreenにした。4回の成功は取得順testで担保済みのため、このtestは5回目の`LingshangWallExhausted`だけを一assertionで検証する。
- 2026-08-11: 「連続する4回の`lingshang_zimo`はindex 135, 134, 133, 132の順を保つ」を選択した。取得位置をindex 135へ固定するmutantで実際値が4枚とも`S0`となるredを確認し、取得回数に応じて減算する実装へ復元してgreenにした。4回の期待列に最初の1回も含まれるため、単独の「最初はindex 135」testは重複として削除し、test list上の両項目を一つのtestで担保する。
- 2026-08-11: 「最初の`lingshang_zimo`はindex 135」を選択し、method未定義でredを確認した。四人用`Bipai`へcrate-privateな消費型操作と取得回数を追加し、index 135から逆順に取得する最小実装でgreenにした。安全境界として4枚上限を`LingshangWallExhausted`、live wall残数0を`LiveWallExhausted`で拒否し、成功時だけ`remaining_count`と取得回数を更新する。
- 2026-08-11: 反復参照testが2回とも一枚の一覧を返すため、「参照は公開枚数を変えない」を完了とした。`reveal_initial_baopai_indicator`の`BipaiSpec`共通implへの移動も確認した。次は上位rule判断と分離できる最初の`lingshang_zimo` index 135を選択した。
- 2026-08-11: 「公開済み初期表ドラ表示牌を複数回参照できる」を選択し、iteratorを空にするmutantで2回とも空になるredを確認して復元後greenにした。2回の参照結果を一配列へ集約して一assertionで比較した。後続の非消費性項目は、公開枚数、通常`zimo`枚数、通常`zimo`位置、`lingshang_zimo`位置という独立して失敗し得る4観点へ分割した。
- 2026-08-11: 「初期表示commandを二回適用すると拒否する」を追加した。guardを外すmutantで二回目が`Ok`となるredを確認し、`InitialBaopaiIndicatorAlreadyRevealed`を返すguardへ復元してgreenにした。
- 2026-08-11: 「初期表示command後はindex 131の一枚」を選択し、command method未定義でredを確認した。`QipaiCompleted`の四人用`Bipai`にcrate-privateな消費型commandを追加し、公開枚数を0から1へ変更してindex 131の`M0`でgreenにした。`Bipai`へrule値は渡していない。
- 2026-08-11: 「`qipai`直後の公開済み表ドラ表示牌は0枚」を選択し、実際値1とのassertion failureでredを確認した。`qipai`が公開枚数を0のまま引き継ぐよう変更してgreenにし、表ドラ無効ruleを表現できる状態にした。
- 2026-08-11: 表ドラ無効ruleを扱うため、`qipai`が初期表示を自動公開する方針を撤回した。`Bipai`は公開済み枚数とindex上限だけを管理し、`Round`がrules crateから変換されたcore所有policyを参照して、表ドラ有効時だけ初期表示commandを適用する責務分担へ更新した。`Bipai`では`qipai`直後0枚、初期表示command後index 131、二重適用拒否、反復参照を順に検証する。
- 2026-08-11: 「`qipai`後の初回取得はindex 131」を選択し、`baopai_indicators`未定義のmethod errorでredを確認した。公開済み表示枚数をconstructorで0、`qipai`後に1として`Bipai`へ保持し、`QipaiCompleted`の四人用`Bipai`だけにread-only iteratorを実装した。fixtureのindex 131へ`M0`を固定してgreenにし、次は反復参照の非消費性を選択した。
- 2026-08-11: 既存の構築直後122枚・`qipai`後70枚の`remaining_count` testにより、末尾14枚を通常`zimo`可能枚数へ含めない項目を完了とした。次に初期表ドラ表示牌のindex 131を選択し、将来の複数表示を同じAPIで扱える`baopai_indicators`から一枚を参照する方針とした。
- 2026-08-11: 三人麻雀では北抜き有効時に嶺上牌8枚、無効時に4枚とする要件を追加した。容量と取得上限は`Bipai` / `Wangpai`、北抜きの合法性と一回分の取得権限発行は`Round`の責務として分離した。三人用実装はPhase 4まで行わない。

## Completion review

- [ ] すべての項目が完了または理由付きで移送されている。
- [ ] 要求 ID と source clause に漏れがない。
- [ ] エラーと境界を確認した。
- [ ] property/model/contract test の要否を判断した。
- [ ] replay と schema/version への影響を確認した。
