# Test list: `Round`の最初の`Dapai`

## Metadata

- Owner: project owner / implementer
- Created: 2026-08-12
- Updated: 2026-08-13
- Status: In progress
- Requirements: `CORE-001`, `CORE-002`, `CORE-006`, `CORE-007`, `CORE-008`, `NFR-001`
- ADR / design: [ADR-0001](../adr/0001-event-driven-typed-continuations.md)、[ADR-0012](../adr/0012-normalize-dealer-first-draw.md)、[ADR-0013](../adr/0013-tile-kind-without-copy-identity.md)、[domain model](../design/domain-model.md)
- Related lists: [最小Player aggregateとRound初期遷移](round-player-initial-state.md)、[雀魂段位戦・四人 walking skeleton](mahjong-soul-ranked-four-player.md)
- Rule sources / clauses: `initial_deal`由来の親第一`Zimo`はADR-0012に従って`moqie = false`とする。

## Scope

四人用のツモ後`Round`から、通常の`Dapai`を一回だけ原子的に適用し、打牌後の反応待ちtypestateへ
遷移するまでを扱う。`Dapai`は分離された`zimopai`を捨てる選択と、`Bingpai`から指定牌を捨てる
選択を区別する。

このlistでは合法action生成、AI request、`Chi`、`Peng`、`Rong`、反応競合の解決、次actorの`Zimo`、
立直宣言を伴う`Dapai`を実装しない。

## Responsibility boundary

- rules crateは親の開始方式を検証し、最初の`Zimo` originをcoreへ渡す。
- `Round`は最初の`Zimo` originとactionの打牌元から`moqie`を導く。
- 呼び出し側は`moqie`を直接指定しない。
- `Player`は`Bingpai`と`He`を所有するが、phaseや`zimopai`を所有しない。
- 将来の`Player`は`LizhiState`も所有する。`Sipai`へ立直宣言牌flagを追加せず、立直状態が追記専用`He`の検証済み`SipaiIndex`を高々一つ保持する。
- `He`への追加、`Bingpai`更新、`zimopai`消費は一つの消費型`Round`遷移で行う。
- 失敗する遷移は消費前の有効な`Round`と型付きerrorを返す。
- 反応待ちphaseのactorは打牌者とし、reactor集合はphase固有dataとして後続項目で追加する。
- `He::is_empty()`から第一打または第一巡の資格を推定しない。配牌後の暗槓または北抜きが第一打より先に行われても意味を失わない明示的なbool flagを状態として保持する。
- playerごとの「まだ第一打前か」と、Round共通の「第一巡の無 interruption 条件が保たれているか」は別の事実として保持する。天和・地和・人和・ダブル立直は、現在phaseとこれらのflagを組み合わせて判定する。
- 暗槓、北抜き、副露などの各actionがどのflagを失わせるかはrule variationを確認し、それぞれのrule testを選択した時点で固定する。

## Examples and tests

### Origin and first-turn state

- [x] `Round`開始時に指定した`FirstZimoOrigin`を、ツモ後typestateが保持する。
- [x] initial deal由来の親の最初の`zimopai`は、originから`moqie = false`になる。
- [ ] **Selected:** live wall由来の親の最初の`zimopai`は、originから判定するpolicyで`moqie = true`になる。
- [ ] 第一打前に暗槓または北抜きがあっても、`He`の空判定ではなくplayerごとの第一打前flagを参照する。
- [ ] 最初の`Dapai`後は、そのplayerの第一打前flagがfalseになる。
- [ ] 他家の第一打前flagを変更せず、打牌によるRound共通の第一巡成立条件だけをruleどおり更新する。

### `zimopai`からの`Dapai`

- [ ] `zimopai`を捨てると、actorの`Bingpai` countsは変わらない。
- [ ] `zimopai`を捨てると、actorの`He`末尾へ同じ`TileKind`が追加される。
- [ ] 成功後のtypestateは`zimopai`を所有しない。
- [ ] 成功後の反応待ちphaseのactorは打牌者である。

### `Bingpai`からの`Dapai`

- [ ] `Bingpai`の牌を捨てると、そのkindが一枚減り、元の`zimopai`が一枚増える。
- [ ] `Bingpai`から捨てた`Sipai`は`moqie = false`になる。
- [ ] `Bingpai`と`zimopai`が同じ`TileKind`でも、打牌元の選択を保持する。
- [ ] 存在しない`Bingpai`の牌を指定すると、元のツモ後`Round`を伴う型付きerrorを返す。

### Atomicity and typestate

- [ ] ツモ後typestateを消費して打牌後の反応待ちtypestateを返す。
- [ ] ツモ前typestateから`Dapai`できない。
- [ ] 打牌後typestateから同じ`Dapai`を連続して行えない。
- [ ] `He`容量違反を不正な部分更新として公開しない。
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

- Selected: live wall由来の親の最初の`zimopai`は、originから判定するpolicyで`moqie = true`になる。
- Phase: Not started
- Why: initial deal由来との対照testで、同じ`Dapai::Moqie`からoriginに応じて公開情報が変わることを確認する。

## Cycle log

- 2026-08-12: 最初の`Dapai`縦切りを設計した。`Zimo` origin、`Dapai`の打牌元、原子的な`Player`更新と反応待ちtypestateを境界とした。
- 2026-08-12: 「親の第一打を常に`moqie = false`とするvariation」は`initial_deal`由来の第一`Zimo`を指しており、ADR-0012の既存判断で表現できると訂正した。追加policyと後継ADR案を削除した。また`actor == zhuangjia && He::is_empty()`では第一打前の暗槓・北抜きを扱えないため、天和・地和・人和・ダブル立直でも使う明示的な第一巡関連bool flagを状態として保持する方針へ修正した。playerごとの第一打前とRound共通の無 interruption 条件は別の事実なので分離する。
- 2026-08-12: 立直宣言牌の識別を追加検討した。高々一回の情報を全`Sipai`へboolとして重複させず、将来`Player`が所有する`LizhiState`から追記専用`He`の検証済み`SipaiIndex`を参照する。宣言牌を追加した応答待ちと立直成立済みを分け、宣言牌への和了可能性を解決する前に成立扱いしない。通常`Dapai`の現在scopeでは実装せず、立直宣言actionの後続項目へ残す。
- 2026-08-12: `Dapai`の`moqie`導出に先立ち、`Round`開始境界から最初の`Zimo` originをツモ後stateへ移送する準備項目を追加した。rules crateの開始方式validationは後続項目とし、この項目ではcoreが検証済みruntime値を保持する契約だけを扱う。
- 2026-08-12: `first_zimo_preserves_configured_origin`を追加した。redでは`FirstZimoOrigin`、`Round::new`の入力、ツモ後stateの観測がなく失敗した。`FourPlayerZimoPending`から`FourPlayerZimoCompleted`へoriginを移送してgreenにし、既存の`Round::new` fixtureも開始方式を明示するよう更新した。
- 2026-08-13: `initial_deal_zimopai_dapai_is_not_moqie`を追加した。`Dapai`、`Round::dapai`、`He`の末尾観測が未定義のcompile errorを、選択項目に対するredとして確認した。
- 2026-08-13: `Dapai::Moqie`と`Dapai::Shouqie(TileKind)`を`round/dapai.rs`へ追加し、`Round::dapai`が打牌後typestateへ遷移する最小実装でgreenにした。initial deal由来では`Sipai::moqie`をfalseとして導出する。失敗時は元のツモ後`Round`を`DapaiFailure`から回収できる。
- 2026-08-13: refactorで大きな`Result` errorを避けるため`DapaiFailure`を`Box`化し、全workspaceのformat、clippy、build、testがgreenであることを確認した。次の対照項目としてlive wall由来を選択した。

## Completion review

- [ ] 最初の`Zimo` originを渡すruntime値のproduction識別子がreview済みである。
- [ ] playerごとの第一打前flagとRound共通の第一巡成立flagの所有位置・更新責務がreview済みである。
- [ ] すべての項目が完了または理由付きで移送されている。
- [ ] `Bingpai`、`zimopai`、`He`の原子性を確認した。
- [ ] 不正actionで消費前の有効状態を失わないことを確認した。
- [ ] event、observation、replayへの`moqie`射影を後続listへ移送した。
- [ ] 立直宣言牌を`Sipai`の重複flagではなく`LizhiState`の`SipaiIndex`から射影することを確認した。
