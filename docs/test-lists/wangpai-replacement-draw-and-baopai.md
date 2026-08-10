# Test list: 王牌・嶺上ツモ・宝牌表示

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-11
- Updated: 2026-08-11
- Status: Planned
- Requirements: `CORE-002`, `CORE-003`, `CORE-005`, `CORE-006`, `CORE-007`, `NFR-001`, `RULE-001`, `RULE-002`, `RULE-010`
- ADR / design: [domain model](../design/domain-model.md), [rules and presets](../design/rules-and-presets.md), [ADR-0013](../adr/0013-tile-kind-without-copy-identity.md)
- Rule sources / clauses: [rule source ledger](../references/rule-sources.md)。service別の槓ドラ公開時点、北抜き、裏ドラ適用条件は実装選択前にsource mappingする。
- Supporting evidence: ユーザー提示のKanachan和了時コードでは、表ドラ表示牌を`131 - i * 2`、裏ドラ表示牌を`130 - i * 2`から取得する。補助資料であり、単独でrule期待値にはしない。

## Scope

四人用136枚`Bipai`の末尾14枚を`wangpai`として扱い、通常`zimo`から分離した嶺上ツモ、
表ドラ表示牌、裏ドラ表示牌の公開状態と反復参照、順序、上限、rule依存の公開時点、tile conservationを扱う。

このlistでは槓の合法性、槍槓の競合、和了計算、点数計算、三人麻雀の北抜き自体の合法性は
実装しない。それらの遷移が発行した検証済みの取得権限を`wangpai`境界が正しく消費することを扱う。

## Decisions

- core内では各幢の上下を再現せず、固定配列末尾側から決定的なindex列で取得する。
- 四人用の嶺上牌は`135, 134, 133, 132`の順とする。
- 表ドラ表示牌は`131, 129, 127, 125, 123`の順とし、最初の1枚と最大4回の追加表示を表す。
- 裏ドラ表示牌は`130, 128, 126, 124, 122`の順とし、対応する表ドラ表示牌と同数だけ取得する。
- 初期の表ドラ表示牌は`qipai`完了後にだけ取得可能とする。配牌前から常時取得できる公開APIは作らない。
- 裏ドラ表示牌は和了成立と裏ドラ適用資格の確定後、点数評価に必要な境界でだけ取得可能とする。進行中の通常viewへ公開しない。
- 公開済みの表裏ドラ表示牌は`wangpai`内の牌へのread-only viewであり、何度参照しても公開枚数や取得位置を変えない。
- 追加の表ドラ表示時点は、上位の`Round`遷移が槓種別とruleから決める。`wangpai`は槓種別やruleを解釈しない。
- 槓と北抜きは`lingshang_zimo`の一回限りの権限を発行する。北抜きは追加の表ドラ表示を発生させない。
- `lingshang_zimo`を1回行うたびに通常`zimo`可能な`remaining_count`も1減らす。物理的な王牌補充によるlive wall短縮を、配列の移動なしでcountへ反映する。
- 通常`zimo`と`lingshang_zimo`で同じindexを二重消費しない。表裏ドラ表示牌は消費せず、対応indexを参照する。

## Responsibility boundary

- `Bipai` / `Wangpai`は、通常`zimo`位置、`lingshang_zimo`位置、公開済み表ドラ枚数、各領域の上限と非重複を管理する。
- 公開済みの表裏ドラ表示牌一覧は何度でも参照でき、参照操作は状態を変更しない。
- `Round`は、`Angang`、`Jiagang`、`Daminggang`、`Babei`の成立、rule、現在phaseから、次に許可する公開・`lingshang_zimo`を決める。
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

- [ ] 四人用`Bipai`の末尾14枚を通常`zimo`可能枚数へ含めない。
- [ ] 通常`zimo`可能範囲、嶺上牌、表ドラ表示牌、裏ドラ表示牌のindex集合は互いに重ならない。
- [ ] Property: 136枚の各indexは、配牌、通常`zimo`可能範囲、嶺上牌、表ドラ表示牌、裏ドラ表示牌のいずれか一つにだけ分類される。

### Initial baopai indicator

- [ ] `qipai`前は初期の表ドラ表示牌を取得できない。
- [ ] `qipai`後の初回取得はindex 131の`TileKind`を返す。
- [ ] 公開済みの初期表ドラ表示牌を複数回参照しても、毎回同じindex 131の`TileKind`を返す。
- [ ] 公開済み表ドラ表示牌の参照は、公開枚数、通常`zimo`位置、`lingshang_zimo`位置を変えない。
- [ ] `qipai`前の参照失敗後も`Bipai`の状態は変化しない。

### Replacement draw authorization

- [ ] 槓成立後に一回分の嶺上ツモ権限を発行できる。
- [ ] 北抜き成立後に一回分の嶺上ツモ権限を発行できる。
- [ ] 槓または北抜きに由来する権限なしでは嶺上ツモできない。
- [ ] 一つの権限から嶺上ツモを二回行えない。
- [ ] 槓用と北抜き用の権限を、原因を失った同一の無型tokenとして扱わない。
- [ ] 失効済みまたは別の`Round`の権限を拒否する。
- [ ] 拒否後も嶺上牌の次の取得位置は変化しない。

### Replacement draw order and limits

- [ ] 最初の嶺上ツモはindex 135の`TileKind`を返す。
- [ ] 連続する4回の嶺上ツモはindex `135, 134, 133, 132`の順を保つ。
- [ ] 4枚取得後は追加の嶺上ツモを拒否する。
- [ ] `lingshang_zimo`を1回行うと通常`zimo`の`remaining_count`も1減る。
- [ ] 4回の`lingshang_zimo`後は通常`zimo`の`remaining_count`が4減る。
- [ ] `lingshang_zimo`を1回行った後、通常`zimo`はindex 120までで終了し、補充対象となったindex 121を返さない。
- [ ] 4回の`lingshang_zimo`後、通常`zimo`はindex 117までで終了し、index 118〜121を返さない。
- [ ] 通常`zimo`は嶺上牌の取得位置を変えない。

### Additional baopai indicators

- [ ] 初期表示後、最初の追加表示はindex 129の`TileKind`を返す。
- [ ] 表ドラ表示牌はindex `131, 129, 127, 125, 123`の順を保つ。
- [ ] 初期表示を含む5枚の表示後は追加表示を拒否する。
- [ ] 追加表示権限なしでは表ドラ表示牌を増やせない。
- [ ] 一つの槓成立から追加の表ドラ表示牌を二枚公開できない。
- [ ] 北抜き後は表ドラ表示枚数が増えず、追加表示権限も発行しない。
- [ ] 公開済みの複数の表ドラ表示牌は何度参照しても同じ順序と内容を返す。
- [ ] 表ドラ表示牌一覧の参照は未処理の追加表示件数を消費しない。

### Round orchestration and rule-dependent reveal timing

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

- Selected: None
- Why this is the smallest useful next test: service別source mappingと槓種別ごとの公開順序ruleを確定してから実装項目を一つ選ぶ。

## Cycle log

実装未着手。

## Completion review

- [ ] すべての項目が完了または理由付きで移送されている。
- [ ] 要求 ID と source clause に漏れがない。
- [ ] エラーと境界を確認した。
- [ ] property/model/contract test の要否を判断した。
- [ ] replay と schema/version への影響を確認した。
