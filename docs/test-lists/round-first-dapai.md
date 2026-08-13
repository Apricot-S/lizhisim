# Test list: `Round`の最初の`Dapai`

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-12
- Updated: 2026-08-13
- Status: In progress
- Requirements: `CORE-001`, `CORE-002`, `CORE-006`, `CORE-007`, `CORE-008`, `NFR-001`
- ADR / design: [ADR-0001](../adr/0001-event-driven-typed-continuations.md)、[ADR-0012](../adr/0012-normalize-dealer-first-draw.md)、[ADR-0013](../adr/0013-tile-kind-without-copy-identity.md)、[ADR-0016](../adr/0016-initial-deal-shouqie-action.md)、[domain model](../design/domain-model.md)
- Related lists: [最小Player aggregateとRound初期遷移](round-player-initial-state.md)、[雀魂段位戦・四人 walking skeleton](mahjong-soul-ranked-four-player.md)
- Rule sources / clauses: `initial_deal`由来の親第一打はADR-0016に従い`Moqie`を提示せず、分離された`zimopai`も`Shouqie`の対象とする。

## Scope

四人用のツモ後`Round`から、通常の`Dapai`を一回だけ原子的に適用し、打牌後の反応待ちtypestateへ
遷移するまでを扱う。live wall等に由来する`zimopai`を捨てる`Moqie`と、`Shouqie`を区別する。
通常の`Shouqie`は`Bingpai`から指定牌を捨てる。14枚配牌を正規化した`initial_deal`由来の親第一打に
限り、分離された`zimopai`も`Shouqie`の対象とする。

このlistでは合法action生成、AI request、`Chi`、`Peng`、`Rong`、反応競合の解決、次actorの`Zimo`、
立直宣言を伴う`Dapai`を実装しない。

## Responsibility boundary

- rules crateは親の開始方式を検証し、最初の`Zimo` originをcoreへ渡す。
- `Round`は`Zimo` originと第一打前の文脈から合法な`Dapai`と牌移動元を決める。
- `Sipai::moqie`はaction variantから導く。`Moqie`は常にtrue、`Shouqie`は常にfalseとし、originによる補正を行わない。
- 呼び出し側は`moqie`を直接指定しない。
- `Dapai::Moqie`と`Dapai::Shouqie`は、上位のaction変換がRound stateなしで対象牌を参照できるよう、どちらも`TileKind`を保持する。
- `initial_deal`由来の親第一打では`Moqie`を合法action候補に含めず、`Shouqie`候補を`Bingpai`と`zimopai`の牌種から生成する。
- `Player`は`Bingpai`と`He`を所有し、`Round`が検証した打牌元を`Player::dapai`が両方へ原子的に適用する。phaseと`Zimo` originは所有しない。
- 将来の`Player`は`LizhiState`も所有する。`Sipai`へ立直宣言牌flagを追加せず、立直状態が追記専用`He`の検証済み`SipaiIndex`を高々一つ保持する。
- `He`への追加、`Bingpai`更新、`zimopai`消費は一つの消費型`Round`遷移で行う。
- 失敗する遷移は型付きerrorを返し、部分更新状態を公開しない。消費済みの`Round`はerrorから回収しない。
- 反応待ちphaseのactorは打牌者とし、reactor集合はphase固有dataとして後続項目で追加する。
- `He::is_empty()`から第一打または第一巡の資格を推定しない。配牌後の暗槓または北抜きが第一打より先に行われても意味を失わない、playerごとの`first_turn_eligible`を状態として保持する。
- `first_turn_eligible`は第一打に限らず、九種九牌、暗槓、北抜き、副露など第一巡の資格へ影響するactionから更新する。通常の`Dapai`ではactorだけをfalseにし、第一巡を中断するactionでは全playerをfalseにする。天和・地和・人和・ダブル立直は、現在phaseとplayerごとのflagを組み合わせて判定する。
- 暗槓、北抜き、副露などの各actionがどのflagを失わせるかはrule variationを確認し、それぞれのrule testを選択した時点で固定する。

## Examples and tests

### Origin and first-turn state

- [x] `Round`開始時に指定した`FirstZimoOrigin`を、ツモ後typestateが保持する。
- [x] initial deal由来の親第一打で`Shouqie(zimopai)`を選ぶと、`Bingpai` countsを変更せず`zimopai`を捨て、`Sipai::moqie = false`になる。
- [x] initial deal由来でもactorが親でなければ、`Shouqie(zimopai)`の特例を適用しない。
- [x] initial deal由来の親第一打で`Moqie`を指定すると、型付きerrorで拒否する。
- [x] live wall由来の親の最初の`zimopai`を`Moqie`すると、`Sipai::moqie = true`になる。
- [x] 第一打前に暗槓または北抜きがあっても、`He`の空判定ではなくplayerごとの`first_turn_eligible`を参照する。
- [x] 最初の`Dapai`後は、そのplayerの`first_turn_eligible`がfalseになる。
- [ ] `fulu`など第一巡を中断するactionでは、全playerの`first_turn_eligible`をfalseへ変更する。

### live wall等の`zimopai`からの`Moqie`

- [x] `zimopai`を捨てると、actorの`Bingpai` countsは変わらない。
- [x] `zimopai`を捨てると、actorの`He`末尾へ同じ`TileKind`が追加される。
- [x] 成功後のtypestateは`zimopai`を所有しない。
- [x] 成功後の反応待ちphaseのactorは打牌者である。

### Later: 合法action候補

- [ ] initial deal由来の親第一打では`Moqie`を候補に含めない。
- [ ] initial deal由来の親第一打では、`Bingpai`と`zimopai`を合わせた牌種を`Shouqie`候補にする。
- [ ] `Bingpai`と`zimopai`が同じ`TileKind`の場合も、同じ意味の`Shouqie`候補を重複させない。
- [ ] live wall由来の`zimopai`には同じ`TileKind`の`Moqie`候補を提示する。

### `He`の観測

- [x] `He::is_empty()`を廃止し、`He`の空・非空をiteratorなどの要素観測APIから判定できる。

### `Bingpai`からの`Dapai`

- [x] `Bingpai`の牌を捨てると、そのkindが一枚減り、元の`zimopai`が一枚増える。
- [x] `Bingpai`から捨てた`Sipai`は`moqie = false`になる。
- [x] `Bingpai`と`zimopai`が同じ`TileKind`でも、打牌元の選択を保持する。
- [x] 存在しない`Bingpai`の牌を指定すると、対象牌を伴う型付きerrorを返す。

### Atomicity and typestate

- [x] ツモ後typestateを消費して打牌後の反応待ちtypestateを返す。
- [x] ツモ前typestateから`Dapai`できない。
- [x] 打牌後typestateから同じ`Dapai`を連続して行えない。
- [ ] **Selected:** `He`容量違反を不正な部分更新として公開しない。
- [ ] Property: 成功する`Dapai`前後で37種類すべての牌保存則を満たす。正式な状態観測または安定hash導入後に実施する。

### Later: 立直宣言牌

- [ ] 通常の`Dapai`は未宣言の`LizhiState`を変更しない。
- [ ] 立直宣言を伴う`Dapai`は、宣言牌を`He`へ追加し、その要素を指す検証済み`SipaiIndex`を応答待ち`LizhiState`へ記録する。
- [ ] 応答待ち`LizhiState`のindexは、`He`へ追加した宣言牌と同じ`TileKind`を指す。
- [ ] 宣言牌への和了可能性を解決する前は、立直成立済みとして扱わない。
- [ ] 宣言牌への和了が成立しなかった場合、同じ`SipaiIndex`を保持した成立済み`LizhiState`へ遷移する。
- [ ] `He`の各`Sipai`へ立直宣言牌flagを重複して保持せず、`LizhiState`から高々一枚を導出する。
- [ ] 応答待ちまたは成立済みのplayerは、二つ目の立直宣言牌を記録できない。

## Current

- Selected: `He`容量違反を不正な部分更新として公開しない。
- Phase: Not started
- Why: `Round::dapai`は`ZimoCompleted`にのみ実装され、打牌後の`DapaiCompleted`には実装されないことを確認したため、次は`HeFull`時に`Round::dapai`が状態を部分更新として公開しないことを検証する。

## Cycle log

- 2026-08-12: 最初の`Dapai`縦切りを設計した。`Zimo` origin、`Dapai`の打牌元、原子的な`Player`更新と反応待ちtypestateを境界とした。
- 2026-08-12: 「親の第一打を常に`moqie = false`とするvariation」は`initial_deal`由来の第一`Zimo`を指しており、ADR-0012の既存判断で表現できると訂正した。追加policyと後継ADR案を削除した。また`actor == zhuangjia && He::is_empty()`では第一打前の暗槓・北抜きを扱えないため、天和・地和・人和・ダブル立直でも使う明示的な第一巡関連bool flagを状態として保持する方針へ修正した。playerごとの第一打前とRound共通の無 interruption 条件は別の事実なので分離する。
- 2026-08-12: 立直宣言牌の識別を追加検討した。高々一回の情報を全`Sipai`へboolとして重複させず、将来`Player`が所有する`LizhiState`から追記専用`He`の検証済み`SipaiIndex`を参照する。宣言牌を追加した応答待ちと立直成立済みを分け、宣言牌への和了可能性を解決する前に成立扱いしない。通常`Dapai`の現在scopeでは実装せず、立直宣言actionの後続項目へ残す。
- 2026-08-12: `Dapai`の`moqie`導出に先立ち、`Round`開始境界から最初の`Zimo` originをツモ後stateへ移送する準備項目を追加した。rules crateの開始方式validationは後続項目とし、この項目ではcoreが検証済みruntime値を保持する契約だけを扱う。
- 2026-08-12: `first_zimo_preserves_configured_origin`を追加した。redでは`FirstZimoOrigin`、`Round::new`の入力、ツモ後stateの観測がなく失敗した。`FourPlayerZimoPending`から`FourPlayerZimoCompleted`へoriginを移送してgreenにし、既存の`Round::new` fixtureも開始方式を明示するよう更新した。
- 2026-08-13: `initial_deal_zimopai_dapai_is_not_moqie`を追加した。`Dapai`、`Round::dapai`、`He`の末尾観測が未定義のcompile errorを、選択項目に対するredとして確認した。
- 2026-08-13: `Dapai::Moqie`と`Dapai::Shouqie(TileKind)`を`round/dapai.rs`へ追加し、`Round::dapai`が打牌後typestateへ遷移する最小実装でgreenにした。initial deal由来では`Sipai::moqie`をfalseとして導出する。失敗時は元のツモ後`Round`を`DapaiFailure`から回収できる。
- 2026-08-13: refactorで大きな`Result` errorを避けるため`DapaiFailure`を`Box`化し、全workspaceのformat、clippy、build、testがgreenであることを確認した。次の対照項目としてlive wall由来を選択した。
- 2026-08-13: originだけでは第二打以降も`moqie = false`になる不具合に対して、`later_zimopai_dapai_is_moqie_when_initial_deal_origin_remains`を追加した。牌山をもう一枚消費した状態で実際値false、期待値trueとなるredを確認した。
- 2026-08-13: `Bipai::is_after_first_zimo`で四人配牌52枚、王牌14枚、最初の`Zimo`1枚を除いた残り69枚かを確認し、initial deal由来かつこの枚数の場合だけ`moqie = false`とする最小修正でgreenにした。全workspaceのformat、clippy、build、testがgreenであることを確認し、live wall由来の対照項目を再選択した。
- 2026-08-13: `Dapai::Moqie(TileKind)`へ変更し、上位変換がRound stateなしで対象牌を参照できるようにした。コンパイラで保証できるvariant payloadの形だけを確認するtestは置かず、既存の`Round::dapai` testで摸切牌を明示するよう更新した。
- 2026-08-13: green状態のrefactorとしてaction値を`round/dapai.rs`から`action.rs`と`action/dapai.rs`へ移した。`Round`が`Player`の`Bingpai`と`He`を外側から置換する経路を削除し、文脈から導いた`zimopai`と`moqie`を添えて`Player::dapai`へaction適用を委譲した。
- 2026-08-13: 上位設計を再確認し、旧状態回収はprotocol/scheduler境界のpending continuationが担い、`Round::dapai`のerror payloadへ旧`Round`を含める必須要件ではないと整理した。旧状態回収用のtransition型とprepared型を削除し、消費型の`Result<NewState, DapaiError>`へ簡素化する。
- 2026-08-13: `live_wall_zimopai_dapai_is_moqie`を追加した。`FirstZimoOrigin::LiveWall`の最初の`zimopai`を摸切すると`Sipai::moqie = true`になる対照testで、initial deal由来との差を固定した。既存のorigin-based導出でgreenとなった。
- 2026-08-13: `player_tracks_first_turn_eligibility_independently_of_empty_he`へ改名した。配牌直後の`He`が空でも、第一巡資格を独立した`Player`状態として観測できる契約を明確化した。
- 2026-08-13: `He::is_empty()`を削除し、`He::iter()`を追加した。空`He`の判定と追加済み`Sipai`の列挙をiteratorで検証し、第一打前状態の判定が`He`の専用空判定APIへ依存しないようにした。
- 2026-08-13: `first_dapai_clears_actor_first_turn_eligibility`へ改名した。actorの最初の`Dapai`成功後に、そのplayerの`first_turn_eligible`だけがfalseになる契約を固定した。
- 2026-08-13: `external_action_can_clear_first_turn_eligibility`へ改名した。`fulu`・`babei`等の後続actionが対象playerへ適用できる消費型のflag更新境界を明確化した。
- 2026-08-13: `shouqie_moves_one_tile_kind_from_bingpai_to_zimopai`を追加した。`Player::dapai`の`Shouqie`が指定`Bingpai`を1枚減らし、文脈の`zimopai`を1枚加える契約を単体testで固定した。
- 2026-08-13: 親の14枚配牌における第一打の仕様認識を訂正し、ADR-0016へ記録した。旧仕様の`initial_deal`由来`Moqie`を`moqie = false`へ補正する項目を廃止した。新仕様では合法候補に`Moqie`を含めず、分離された`zimopai`も`Shouqie`の対象とする。`Moqie`は常に`moqie = true`、`Shouqie`は常に`moqie = false`とし、既存の`initial_deal_zimopai_dapai_is_not_moqie`とorigin依存補正testは後続実装で置き換える。
- 2026-08-13: 第一巡中断時の状態をRound共通flagではなくplayerごとの`first_turn_eligible`へ統一した。通常の`Dapai`はactorだけをfalseにし、`fulu`等のinterruptionは全playerをfalseにする。以前追加した対象playerだけを外部からfalseにするtestは、具体的な中断actionのtestで置き換える。
- 2026-08-13: `initial_deal_zimopai_can_be_shouqie`へ旧testを置き換えた。redでは`Shouqie(P5)`が`Bingpai`に存在しないため失敗した。`Round`がorigin、actorの`first_turn_eligible`、`zimopai`との一致から検証済み`PlayerDapai`へ変換し、`ShouqieFromZimopai`が`Bingpai`を変更せず`moqie = false`の`Sipai`を追加してgreenにした。refactorで`Sipai::moqie`をaction由来へ統一し、旧origin補正専用の`Bipai::is_after_first_zimo`とtestを削除した。
- 2026-08-13: `initial_deal_zimopai_shouqie_exception_requires_zhuangjia_actor`を追加した。redではinitial deal originと第一巡資格だけで非親actorにも`ShouqieFromZimopai`特例が適用された。特例guardへ`actor == zhuangjia`を追加し、残り枚数ではなく親第一打というdomain条件を直接表現してgreenにした。
- 2026-08-13: `initial_deal_first_dapai_rejects_moqie`を追加した。`MoqieUnavailableForInitialDealFirstDapai`を型付きerrorとして追加し、initial deal由来・親actor・第一巡資格の条件を満たす`Moqie`を`Player`更新前に拒否してgreenにした。
- 2026-08-13: `moqie_preserves_actor_bingpai_counts`を追加した。既存の`PlayerDapai::Moqie`は`Bingpai`を変更しないため、counts全体を比較する回帰testとしてgreenを確認した。
- 2026-08-13: `moqie_appends_zimopai_tile_kind_to_actor_he`を追加した。live wall由来の通常`Moqie`後、actorの`He`末尾に`zimopai`と同じ`TileKind`が追加される既存挙動を回帰testとして固定した。
- 2026-08-13: `DapaiCompleted`はunit structで`zimopai`を保持せず、`Round<FourPlayer, ZimoCompleted>`だけが`zimopai()`を公開することをAPI reviewで確認した。コンパイラが保証する形状のため、`compile_error!()`やcompile-fail testは追加しない。
- 2026-08-13: `dapai_completed_actor_is_dapai_actor`を追加した。親以外のseatをactorとして再構成したfixtureで`DapaiCompleted`のactorが同じ打牌者として移送される既存挙動を回帰testで固定した。
- 2026-08-13: `shouqie_appends_non_moqie_sipai`を追加した。`PlayerDapai::ShouqieFromBingpai`が指定牌種の`Sipai`を`moqie = false`で追加する既存挙動を単体testで固定した。
- 2026-08-13: `same_tile_kind_keeps_moqie_and_shouqie_distinct`を追加した。`Bingpai`と`zimopai`がともに`P5`でも、`Moqie(P5)`は`moqie = true`、`ShouqieFromBingpai(P5, P5)`は`moqie = false`として記録し、打牌元の選択を保持することを単体testで固定した。
- 2026-08-13: `shouqie_from_absent_bingpai_tile_reports_tile_kind`を追加した。`Bingpai`にない`Z4`を`ShouqieFromBingpai`へ指定すると、対象牌を持つ`DapaiError::Bingpai(BingpaiError::TileNotPresent)`を返す既存挙動を単体testで固定した。
- 2026-08-13: `Round<FourPlayer, ZimoCompleted>::dapai(self, Dapai) -> Result<Round<FourPlayer, DapaiCompleted>, DapaiError>`の公開APIをreviewし、成功時にツモ後typestateを消費して打牌後typestateを返すことを確認した。`Round<FourPlayer, ZimoPending>`には`dapai` implがないため、ツモ前typestateから打牌できないことも同じAPI形状で確認した。コンパイラが保証するためcompile-fail testは追加しない。
- 2026-08-13: `Round::dapai`は`Round<FourPlayer, ZimoCompleted>`にのみ実装され、成功時の戻り値である`Round<FourPlayer, DapaiCompleted>`には同名メソッドがないことをreviewした。したがって打牌後typestateから同じ`Dapai`を連続して行えず、コンパイラが保証するためcompile-fail testは追加しない。

## Completion review

- [ ] 最初の`Zimo` originを渡すruntime値のproduction識別子がreview済みである。
- [ ] playerごとの`first_turn_eligible`の所有位置・更新責務がreview済みである。
- [ ] すべての項目が完了または理由付きで移送されている。
- [ ] `Bingpai`、`zimopai`、`He`の原子性を確認した。
- [ ] 不正actionで部分更新状態を公開せず、具体的な型付きerrorを返すことを確認した。
- [ ] event、observation、replayへの`moqie`射影を後続listへ移送した。
- [ ] initial deal由来の親第一打における合法action候補生成を後続listへ移送した。
- [ ] 立直宣言牌を`Sipai`の重複flagではなく`LizhiState`の`SipaiIndex`から射影することを確認した。
